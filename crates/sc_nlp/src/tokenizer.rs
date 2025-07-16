// /shyen_capital/crates/sc_nlp/src/tokenizer.rs

use tokenizers::{Tokenizer, Encodable, Encoding};
use std::path::Path;

/// A wrapper around the Hugging Face Tokenizer library, configured specifically
/// for Shyen Capital's needs.
pub struct ShyenTokenizer {
    tokenizer: Tokenizer,
}

impl ShyenTokenizer {
    /// Loads a tokenizer from a `tokenizer.json` file.
    ///
    /// This file contains the vocabulary and tokenization rules from a pre-trained
    /// model like FinBERT.
    ///
    /// # Arguments
    /// * `model_path` - The path to the directory containing the `tokenizer.json` file.
    pub fn new(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let vocab_path = Path::new(model_path).join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(vocab_path)
           .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        
        Ok(Self { tokenizer })
    }

    /// Encodes a batch of texts into a format the model can understand.
    ///
    /// # Arguments
    /// * `texts` - A slice of strings to encode.
    ///
    /// # Returns
    /// A vector of `Encoding` objects, which contain the token IDs and attention masks.
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Encoding> {
        // The `encode_batch` method from the `tokenizers` crate is highly optimized
        // and can process many texts in parallel.
        self.tokenizer.encode_batch(texts.to_vec(), true)
           .unwrap_or_else(|e| {
                eprintln!(": {}", e);
                vec!
            })
    }
}