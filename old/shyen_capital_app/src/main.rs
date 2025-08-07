// /shyen_capital/shyen_capital_app/src/main.rs

use sc_core::DataPacket;
use sc_data_ingestors::start_ingestors;
use sc_nlp::SentimentAnalyzer;
use tokio::sync::mpsc;

/// The main entry point for the Shyen Capital Trading Engine.
#[tokio::main]
async fn main() {
    // --- Step 1: Load Environment Variables ---
    // This is the crucial step. It loads the variables from the.env file
    // into the application's environment *before* any other code runs.
    // This ensures that HF_HOME is set when the tokenizer is initialized.
    dotenvy::dotenv().expect("Failed to load.env file");
    println!(": Environment variables loaded.");

    // --- Step 2: Initialize Core Components ---
    // Create the central data bus (an MPSC channel).
    // MPSC = Multi-Producer, Single-Consumer. Many ingestors can send data,
    // but one central engine will consume it.
    let (tx, mut rx) = mpsc::channel::<DataPacket>(1000);

    // Initialize the Sentiment Analyzer. This will now use the HF_HOME variable
    // to determine where to download and cache the ModernFinBERT tokenizer.
    let sentiment_analyzer = SentimentAnalyzer::new().expect("Failed to initialize Sentiment Analyzer");
    println!(": Sentiment Analyzer initialized successfully.");

    // --- Step 3: Start Data Ingestion Streams ---
    // This function will spawn all our data ingestors (market data, news, social)
    // as concurrent tasks. They will immediately start feeding data into our `tx` channel.
    start_ingestors(tx).await;

    // --- Step 4: The Main Processing Loop ---
    // This loop represents the core of the trading engine. It will continuously
    // receive data packets from the data bus and process them.
    println!(": Entering main processing loop. Waiting for data packets...");
    while let Some(packet) = rx.recv().await {
        match packet {
            DataPacket::Market(tick) => {
                // In the future, this is where we'll update our price charts,
                // check for trading signals, etc.
                // For now, we'll just log that we received it.
                // Note: We convert the fixed-size byte array back to a string for display.
                let symbol = std::str::from_utf8(&tick.symbol).unwrap_or("").trim_end_matches('\0');
                println!(": Received Market Tick for {}: Price {}", symbol, tick.price);
            }
            DataPacket::Sentiment(point) => {
                // This is where we would use our NLP engine.
                // The ingestor would send raw text, and here we would call:
                // let score = sentiment_analyzer.analyze(&[point.text]);
                println!(": Received raw sentiment data from source: {}", point.source);
            }
        }
    }
}