use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    content: String,
    author: String,
}

#[embassy_executor::task]
async fn network_task() {
    loop {
        match fetch_quote().await {
            Ok(quote) => {
                info!("Quote: \"{}\" - {}", quote.content, quote.author);
            }
            Err(e) => {
                warn!("Failed to fetch quote: {}", e);
            }
        }

        Timer::after(Duration::from_secs(10)).await;
    }
}

async fn fetch_quote() -> Result<Quote, Box<dyn std::error::Error>> {
    // Use a blocking client in a separate thread to avoid runtime conflicts
    let handle = std::thread::spawn(|| {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        client
            .get("https://api.quotable.io/random")
            .send()?
            .json::<Quote>()
    });

    let quote = handle.join().map_err(|_| "Thread panicked")??;
    Ok(quote)
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
