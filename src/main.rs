mod mapping_config;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use log::{info, warn};

use mapping_config::MappingConfig;

#[embassy_executor::task]
async fn network_task() {
    let url =
        "https://raw.githubusercontent.com/johnhalz/roverOS/refs/heads/main/config/mapping.toml";

    loop {
        match fetch_quote(&url).await {
            Ok(config_text) => {
                let mapping_config =
                    MappingConfig::from_toml(&config_text).expect("Failed to parse mapping config");
                info!("Config: {:?}", mapping_config);
            }
            Err(e) => {
                warn!("Failed to fetch config: {}", e);
            }
        }

        Timer::after(Duration::from_secs(10)).await;
    }
}

async fn fetch_quote(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Use a blocking client in a separate thread to avoid runtime conflicts
    let url = url.to_string();
    let handle = std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        client.get(&url).send()?.text()
    });

    let config_text = handle.join().map_err(|_| "Thread panicked")??;
    Ok(config_text)
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting Tasks... (Press Ctrl+C to stop)");

    // Spawn all tasks concurrently
    spawner.spawn(network_task()).unwrap();
}
