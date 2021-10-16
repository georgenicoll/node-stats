// console.log(process.env);
let env_config = {
    backend_dns_address: process.env["BACKEND_DNS_ADDRESS"] || 'localhost',
    backend_port: parseInt(process.env["BACKEND_PORT"] || "9000")
};
console.log(`Environment config set to: ${ JSON.stringify(env_config) }`);

export { env_config };