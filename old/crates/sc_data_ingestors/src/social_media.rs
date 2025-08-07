// /shyen_capital/crates/sc_data_ingestors/src/social_media.rs

use sc_core::{DataPacket, SentimentDataPoint};
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};
use roux::Reddit;
use roux::submission::Submission;
use std::env;

// Define the target subreddits as a constant for easy configuration.
const TARGET_SUBREDDITS: &[&str] = &["wallstreetbets", "stocks", "investing"];

/// Connects to the Reddit API and streams new comments from target subreddits.
///
/// This function authenticates with Reddit, then enters a loop to periodically
/// fetch the latest comments, creating a near real-time data feed.
///
/// # Arguments
/// * `tx` - The sender channel to push sentiment data into.
pub async fn stream_reddit_data(tx: Sender<DataPacket>) -> Result<(), Box<dyn std::error::Error>> {
    // --- Reddit API Authentication ---
    // It is best practice to load credentials from environment variables.
    // This avoids hardcoding sensitive information into the source code.
    // The `.expect()` method will cause the program to panic if the variable is not set,
    // which is appropriate here as the module cannot function without credentials.
    let client_id = env::var("REDDIT_CLIENT_ID").expect("REDDIT_CLIENT_ID not set in environment");
    let client_secret = env::var("REDDIT_CLIENT_SECRET").expect("REDDIT_CLIENT_SECRET not set in environment");
    let username = env::var("REDDIT_USERNAME").expect("REDDIT_USERNAME not set in environment");
    let password = env::var("REDDIT_PASSWORD").expect("REDDIT_PASSWORD not set in environment");

    println!(": Authenticating with Reddit API...");
    
    // Create a new Reddit client and log in.
    let client = Reddit::new("shyen_capital_v0.1 by /u/neilio3264", &client_id, &client_secret)
       .username(&username)
       .password(&password)
       .login()
       .await?;

    println!(": Authentication successful. Starting stream...");

    // Set a polling interval to fetch new comments every 10 seconds.
    // This is a respectful rate that should not violate API rate limits.
    let mut ticker = interval(Duration::from_secs(10));

    loop {
        ticker.tick().await;

        for subreddit_name in TARGET_SUBREDDITS {
            let subreddit = client.subreddit(subreddit_name);
            // Fetch the 10 most recent comments from the subreddit.
            if let Ok(comments) = subreddit.latest_comments(None, Some(10)).await {
                for comment in comments.data.children {
                    // The actual comment text is in the `body` field.
                    if let Some(text) = comment.data.body {
                        // In the future, this text will be sent to the sc_nlp crate for scoring.
                        // For now, we will print it to verify the data stream is working.
                        println!(" r/{}: {}", subreddit_name, text.chars().take(80).collect::<String>());

                        // --- Placeholder for sending to the data bus ---
                        // let sentiment_point = SentimentDataPoint {
                        //     source: 1, // 1 represents Reddit
                        //     score: 0.0, // This will be calculated by the NLP module
                        //     timestamp_ns: 0, // Generate a real timestamp here
                        // };
                        //
                        // if tx.send(DataPacket::Sentiment(sentiment_point)).await.is_err() {
                        //     eprintln!(": Receiver closed. Shutting down.");
                        //     return Ok(());
                        // }
                        // --- End Placeholder ---
                    }
                }
            }
        }
    }
}