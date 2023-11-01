use serde::Deserialize;
use toml;

pub fn read_config() {
    
}

#[derive(Deserialize)]
struct Config {
    server: Server,
    keys: Keys,
}

#[derive(Deserialize)]
struct Server {
    port: u16,
    host: String,
    debug: bool,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    let toml_bytes = std::fs::read("config.toml").expect("Failed to read file");

    let config: Config = toml::from_str(&toml_bytes.to_owned().as_slice()).expect("Invalid TOML format");

    println!("Server port: {}", config.server.port);
    println!("Server host: {}", config.server.host);
    println!("Server debug: {}", config.server.debug);
    println!("Keys github: {}", config.keys.github);
    println!("Keys travis: {:?}", config.keys.travis);
}
