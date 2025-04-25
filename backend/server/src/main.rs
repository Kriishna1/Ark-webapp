mod core;
mod api;

use std::fs;
use std::io;
use toml;

fn main() -> io::Result<()> {
    // Initialize logging
    core::logger::setup_logger();

    // Read and parse configuration file
    let config_content = fs::read_to_string("ark.config.toml").map_err(|e| {
        eprintln!("Configuration file error: {}", e);
        io::Error::new(io::ErrorKind::Other, "Failed to read configuration file")
    })?;

    let app_config: core::config::AppConfig = toml::from_str(&config_content).map_err(|e| {
        eprintln!("Configuration parsing error: {}", e);
        io::Error::new(io::ErrorKind::Other, "Invalid configuration format")
    })?;

    // Launch server in async runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            core::server::launch_api_server(app_config).await
        })
}
