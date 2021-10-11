import { Command } from 'commander/esm.mjs';
import fs from 'fs';
import http from 'http';
import url from 'url';
import { exec as cp_exec } from 'child_process';
import util from 'util';
import { exit } from 'process';

const program = new Command();
program
    .option('-p, --port <number>', 'port to listen on', "9000")
    .option('-c, --commandsfile <string>', 'commands file to load');
program.parse(process.argv);

const options = program.opts();

if (options.commandsfile === undefined) {
    console.log('-c (--commandsfile) argument is required');
    exit(-1);
}

console.log('Loading commands from %s', options.commandsfile);
const commands = JSON.parse(fs.readFileSync(options.commandsfile, 'utf-8'));
const port = parseInt(options.port);

const exec = util.promisify(cp_exec);

const app = http.createServer(async (req, resp) => {
    const parsedUrl = url.parse(req.url, true);

    //Check for GET /api/stats
    if (req.method === 'GET') {
        if (parsedUrl.pathname === '/api/stats')  {
            getStats(resp);
            return;
        }
        if (parsedUrl.pathname === '/api/health') {
            getHealth(resp);
            return;
        }
    } 

    resp.statusCode = 400;
    resp.end("Unsupported");
});

console.log('Listening on port %d', port);
app.listen(port);

async function getStats(resp) {
    const promises = commands.map(({ key, command }) => {
        return exec(command).then(({ stdout, stderr }) => {
            var value = stdout.trim();
            const err = stderr.trim();
            if (err.length > 0) {
                value = value + "|stderr:" + err;
            }
            return {
                key: key,
                value: value
            };
        });
    });
    const promisedStats = Promise.all(promises);

    resp.statusCode = 200;
    resp.setHeader('content-type', 'application/json');
    resp.end(JSON.stringify(await promisedStats));
}

function getHealth(resp) {
    resp.statusCode = 200;
    resp.setHeader('content-type', 'text/plain');
    resp.end('OK');
}