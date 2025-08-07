// /shyen_capital/crates/sc_data_ingestors/src/lib.rs

// Declare the sub-modules that will contain the logic for each data source.
// This keeps our code organized and modular.
pub mod market_data;
pub mod social_media;

use sc_core::DataPacket;
use tokio::sync::mpsc::Sender;

/// The main entry point for the data ingestion system.
/// This function spawns separate, concurrent tasks for each data source.
/// Each task will run independently, feeding its data into the provided channel.
///
/// # Arguments
/// * `tx` - A Tokio MPSC sender channel to push `DataPacket`s into the central data bus.
pub async fn start_ingestors(tx: Sender<DataPacket>) {
    println!(": Starting all data streams...");

    // Spawn a new asynchronous task for the real-time market data feed.
    // We clone the sender `tx` so each task has its own handle to the channel.
    let market_tx = tx.clone();
    tokio::spawn(async move {
        // We wrap the stream in a loop to handle automatic reconnections.
        loop {
            if let Err(e) = market_data::stream_market_data(market_tx.clone()).await {
                eprintln!("[Market Ingestor]: Error: {}. Reconnecting in 5 seconds...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    });

    // Spawn a task for the Wall Street Journal+ web scraper.
    let news_tx = tx.clone();
    tokio::spawn(async move {
        // The scraper will run on a fixed interval (e.g., every 5 minutes)
        // to check for new articles without overwhelming the site.
        news_scraper::run_scraper_loop(news_tx).await;
    });

    // Spawn a task for the Reddit stream.
    let reddit_tx = tx.clone();
    tokio::spawn(async move {
        // This stream will handle reconnections internally.
        if let Err(e) = social_media::stream_reddit_data(reddit_tx).await {
            eprintln!(": Stream failed critically: {}", e);
        }
    });

    // We can add more ingestors here (e.g., for news, options data) following the same pattern.
}