// /shyen_capital/crates/sc_data_ingestors/src/news_scraper.rs

use reqwest::Client;
use scraper::{Html, Selector};
use sc_core::{DataPacket, SentimentDataPoint}; // We will use these later
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};

/// Runs a continuous loop to scrape news sites at a set interval.
///
/// # Arguments
/// * `tx` - The sender channel to push scraped data into.
pub async fn run_scraper_loop(tx: Sender<DataPacket>) {
    // Create a single, reusable HTTP client with a cookie store to maintain login sessions.
    let client = Client::builder()
       .cookie_store(true)
       .build()
       .expect("Failed to build HTTP client");

    // --- Placeholder for Login Logic ---
    // In a real application, you would perform a POST request to the login form
    // of WSJ.com here, which would populate the client's cookie store.
    // This is a complex step that requires inspecting the website's network traffic
    // to identify the correct form data and endpoint.
    println!(": Performing initial login (placeholder)...");
    // e.g., login_to_wsj(&client).await;
    // --- End Placeholder ---

    // Set the scraper to run every 5 minutes. This is a respectful interval
    // that avoids overwhelming the target sites.
    let mut ticker = interval(Duration::from_secs(300));

    loop {
        ticker.tick().await;
        println!(": Scraping WSJ, Barron's, and MarketWatch...");

        // In a full implementation, you would have separate functions for each site.
        // For now, we'll simulate scraping one.
        if let Ok(articles) = scrape_site(&client, "https://www.wsj.com").await {
            for article_text in articles {
                // For now, we just print. Later, this text will be sent to the sc_nlp crate
                // to be converted into a SentimentDataPoint.
                println!(": Found article - {}", article_text);
                
                // Example of sending a placeholder sentiment point:
                // let sentiment_point = SentimentDataPoint { source: 2, score: 0.0, timestamp_ns: 0 };
                // if tx.send(DataPacket::Sentiment(sentiment_point)).await.is_err() {
                //     eprintln!(": Receiver closed. Shutting down.");
                //     return;
                // }
            }
        }
    }
}

/// Scrapes a given URL for article headlines.
///
/// # Arguments
/// * `client` - A reference to the reqwest Client.
/// * `url` - The URL of the site to scrape.
async fn scrape_site(client: &Client, url: &str) -> Result<Vec<String>, reqwest::Error> {
    let html_content = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&html_content);

    // This selector is an EXAMPLE. You must inspect the website's HTML using browser
    // developer tools to find the correct CSS selector for headlines (e.g., 'h2.wsj-headline a').
    // This is the most fragile part of the scraper and may need frequent updates.
    let headline_selector = Selector::parse("h3").unwrap();

    let headlines: Vec<String> = document
       .select(&headline_selector)
       .map(|element| element.text().collect::<String>())
       .collect();

    Ok(headlines)
}