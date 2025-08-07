// /shyen_capital/crates/sc_nlp/src/tokenizer.rs

use tokenizers::{Tokenizer, Encodable, Encoding};

/// A wrapper around the Hugging Face Tokenizer library, configured specifically
/// for the `tabularisai/ModernFinBERT` model.
pub struct ShyenTokenizer {
    tokenizer: Tokenizer,
}

impl ShyenTokenizer {
    /// Loads the tokenizer for `tabularisai/ModernFinBERT` from the Hugging Face Hub.
    ///
    /// The library will handle downloading the tokenizer configuration and caching it
    /// locally for subsequent runs, ensuring performance and consistency.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // The model identifier string is the key. This tells the library exactly
        // which tokenizer to fetch from the Hugging Face Hub.
        let model_identifier = "tabularisai/ModernFinBERT";
        
        let tokenizer = Tokenizer::from_pretrained(model_identifier, None)
          .map_err(|e| format!("Failed to load tokenizer for {}: {}", model_identifier, e))?;
        
        Ok(Self { tokenizer })
    }

    /// Encodes a batch of texts into a format the model can understand.
    ///
    - A vector of `Encoding` objects, which contain the token IDs and attention masks.
    pub fn encode_batch(&self, texts: &[&str]) -> Vec<Encoding> {
        // The `encode_batch` method from the `tokenizers` crate is highly optimized
        // and can process many texts in parallel using multiple threads.
        self.tokenizer.encode_batch(texts.to_vec(), true)
          .unwrap_or_else(|e| {
                eprintln!(": {}", e);
                vec!
            })
    }
}