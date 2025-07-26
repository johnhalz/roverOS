use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

// Shared counter for demonstration
static COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    content: String,
    author: String,
}

#[embassy_executor::task]
async fn periodic_counter() {
    loop {
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        info!("Counter task: {}", count);
        Timer::after(Duration::from_secs(2)).await;
    }
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

#[embassy_executor::task]
async fn file_watcher() {
    use std::fs;

    let log_file = "embassy_demo.log";

    loop {
        let count = COUNTER.load(Ordering::Relaxed);
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] Counter value: {}\n", timestamp, count);

        if let Err(e) = fs::write(log_file, &log_entry) {
            warn!("Failed to write to log file: {}", e);
        } else {
            info!("Logged counter value to file");
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn system_monitor() {
    loop {
        // Simulate system monitoring
        let load = get_system_load().await;
        info!("System load: {:.2}", load);

        if load > 0.8 {
            warn!("High system load detected!");
        }

        Timer::after(Duration::from_secs(3)).await;
    }
}

async fn get_system_load() -> f64 {
    // Simple mock system load - in real app you'd use system APIs
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let load = (now.as_secs() % 100) as f64 / 100.0;
    load
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("🦀 Starting Embassy executor on macOS");
    info!("Press Ctrl+C to stop");

    // Spawn all tasks concurrently
    spawner.spawn(periodic_counter()).unwrap();
    spawner.spawn(network_task()).unwrap();
    spawner.spawn(file_watcher()).unwrap();
    spawner.spawn(system_monitor()).unwrap();

    // Main application loop
    let mut iteration = 0;
    loop {
        iteration += 1;
        info!("🔄 Main loop iteration: {}", iteration);

        // Check if we should do any maintenance
        if iteration % 10 == 0 {
            let total_count = COUNTER.load(Ordering::Relaxed);
            info!("📊 Total counter increments: {}", total_count);
        }

        Timer::after(Duration::from_secs(8)).await;
    }
}
