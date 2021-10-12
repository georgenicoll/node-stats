use simple_logger::SimpleLogger;
#[macro_use]
extern crate clap;
extern crate execute;
use async_trait::async_trait;
use execute::shell;
use futures::future::join_all;
use serde::Serialize;
use std::fs;
use std::process::{Child, Stdio};
use std::str;
use std::sync::Arc;
use std::time::Duration;
use tide::prelude::*;
use tide::{Endpoint, Error, Request };

#[macro_use]
extern crate log;

const LOG_TARGET: &str = "stats-backend";

#[derive(Debug, Serialize)]
struct Stat {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct CommandDetails {
    key: String,
    command: String,
}

#[derive(Debug)]
struct Config {
    port: u16,
    commands: Vec<CommandDetails>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    match parse_arguments() {
        Err(error) => {
            error!("Config load failed: {}", error);
            match clap_arg_matches().print_help() {
                Err(error) => error!("Error printing help message: {}", error),
                Ok(()) => (),
            }
            Err(tide::Error::from_str(500, error))
        }
        Ok(config) => start_server(Arc::new(config)).await,
    }
}

fn parse_arguments() -> Result<Config, String> {
    let matches = clap_arg_matches().get_matches();

    let port = matches
        .value_of("PORT")
        .map(|port_string| port_string.parse::<u16>().unwrap())
        .unwrap_or(9000u16);
    let commands: Result<Vec<CommandDetails>, String> = matches
        .value_of("COMMANDS_FILE")
        .ok_or("--commands(-c) option must be specified".to_string())
        .and_then(load_commands);

    commands.map(|commands| Config { port, commands })
}

fn load_commands(command_file: &str) -> Result<Vec<CommandDetails>, String> {
    fs::read_to_string(command_file)
        .map_err(|error| error.to_string())
        .and_then(|commands_string| {
            serde_json::from_str::<Vec<CommandDetails>>(&commands_string)
                .map_err(|serde_err| serde_err.to_string())
        })
}

fn clap_arg_matches<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(backend =>
        (@arg COMMANDS_FILE: -c --commands-file +takes_value "the commands file to use (required)")
        (@arg PORT: -p --port +takes_value "the port to listen on (default 9000)")
    )
    .bin_name("stats-backend-rust")
}

async fn start_server(config: Arc<Config>) -> tide::Result<()> {
    let address = "0.0.0.0";
    let mut app = tide::new();

    app.at("/api/health").get(get_health);

    let get_stats = GetStats::new(Arc::clone(&config));
    app.at("/api/stats").get(get_stats);

    // .get(move |_req: Request<()>| async move { get_stats(config_clone).await });
    info!(target: LOG_TARGET, "Will Listen on {}", config.port);
    app.listen(format!("{}:{}", address, config.port)).await?;
    Ok(())
}

async fn get_health(_req: Request<()>) -> tide::Result<tide::Response> {
    Ok("OK".to_string()).map(convert_to_text_response)
}

struct GetStats {
    config: Arc<Config>   
}

impl GetStats {
    fn new(config: Arc<Config>) -> GetStats {
        GetStats { config }
    }
}

#[async_trait]
impl Endpoint<()> for GetStats {
    async fn call(&self, req: Request<()>) -> tide::Result<tide::Response> {
        get_stats(req, &self.config).await.map(convert_to_json_response)
    }
}

async fn get_stats(_req: Request<()>, config: &Config) -> tide::Result<String> {
    let stat_futures: Vec<_> = config.commands.iter().map(get_stat_for_command).collect();
    let done_futures = join_all(stat_futures).await;
    match serde_json::to_string(&done_futures) {
        Ok(payload) => Ok(payload),
        Err(why) => Err(Error::from(why)),
    }
}

async fn get_stat_for_command(command: &CommandDetails) -> Stat {
    let spawned = shell(&command.command)
        .stdout(Stdio::piped())
        .spawn();
    match spawned {
        Err(error) => Stat {
            key: command.key.clone(),
            value: format!("Couldn't spawn child process: {}", error),    
        },
        Ok(child) => wait_for_process(command, child).await
    }
}

async fn wait_for_process(command: &CommandDetails, mut child: Child) -> Stat {
    loop {
        async_std::task::sleep(Duration::from_millis(20)).await;
        match child.try_wait() {
            Ok(None) => (),
            Ok(Some(_)) => break,
            Err(_) => break,
        }
    }
    match child.wait_with_output() {
        Err(error) => Stat {
            key: command.key.clone(),
            value: format!("Error: {}", error),
        },
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or("Failed").trim_end().to_string();
            Stat {
                key: command.key.clone(),
                value: stdout,
            }                    
        }
    }
}

fn convert_to_text_response(result: String) -> tide::Response {
    convert_to_response(result, tide::http::mime::PLAIN)
}

fn convert_to_json_response(result: String) -> tide::Response {
    convert_to_response(result, tide::http::mime::JSON)
}

fn convert_to_response(result: String, content_type: impl Into<tide::http::mime::Mime>) -> tide::Response {
    tide::Response::builder(200)
        .content_type(content_type)
        .body(result)
        .build()
}
