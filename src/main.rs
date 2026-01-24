// src/main.rs
mod config;

use config::Config;

fn main() {
    dotenvy::dotenv().ok();

    match Config::from_env() {
        Ok(config) => println!("Config loaded: port={}", config.port),
        Err(e) => eprintln!("Config error: {}", e),
    }
}
