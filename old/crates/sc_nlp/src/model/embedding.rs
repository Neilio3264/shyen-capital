// /shyen_capital/crates/sc_nlp/src/model/embedding.rs

use ndarray::{Array1, Array2, Axis};
use sc_core::DataPacket; // We might use this later, good practice to import.

/// Represents the complete embedding layer for a BERT-style model.
///
/// This layer is responsible for converting input token IDs into high-dimensional
/// vectors that encode meaning, position, and token type. It is the first
/// and most critical layer in our neural network.
pub struct ShyenEmbeddings {
    // These would be large matrices loaded from the trained model files.
    token_embeddings: Array2<f32>,
    position_embeddings: Array2<f32>,
    token_type_embeddings: Array2<f32>,
    
    // Parameters for the Layer Normalization step.
    layer_norm_gamma: Array1<f32>,
    layer_norm_beta: Array1<f32>,
}

impl ShyenEmbeddings {
    /// Creates a new instance of the embedding layer.
    /// In a real application, this function would load the weight matrices
    /// from the downloaded ModernFinBERT model files.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // --- Placeholder for loading model weights ---
        // For now, we'll initialize with empty arrays for architectural purposes.
        // The dimensions (e.g., vocab_size, hidden_size) would be read from a model config file.
        let vocab_size = 30522; // Standard for BERT-base models
        let max_position_embeddings = 512; // Max sentence length
        let type_vocab_size = 2; // For sentence A vs sentence B
        let hidden_size = 768; // The dimensionality of the vectors

        Ok(Self {
            token_embeddings: Array2::zeros((vocab_size, hidden_size)),
            position_embeddings: Array2::zeros((max_position_embeddings, hidden_size)),
            token_type_embeddings: Array2::zeros((type_vocab_size, hidden_size)),
            layer_norm_gamma: Array1::ones(hidden_size),
            layer_norm_beta: Array1::zeros(hidden_size),
        })
    }

    /// Performs the forward pass of the embedding layer.
    ///
    /// # Arguments
    /// * `input_ids` - A 2D array of token IDs (batch_size x sequence_length).
    ///
    /// # Returns
    /// A 3D array of the final embeddings (batch_size x sequence_length x hidden_size).
    pub fn forward(&self, input_ids: &Array2<u32>) -> Array2<f32> {
        // --- Step 1: Get Token Embeddings ---
        // For each ID in the input, look up its corresponding vector in the token_embeddings table.
        // This is a placeholder for an efficient lookup operation.
        let mut token_embeds = Array2::zeros(input_ids.raw_dim());
        // In a real implementation, this would be a highly optimized `gather` operation.

        // --- Step 2: Get Positional Embeddings ---
        // Create positional IDs (0, 1, 2,...) and look up their embeddings.
        let seq_length = input_ids.shape()[1];
        let position_embeds = self.position_embeddings.slice(s![..seq_length,..]);

        // --- Step 3: Get Token Type Embeddings ---
        // For sentiment analysis, all tokens are type 0.
        let token_type_embeds = self.token_type_embeddings.slice(s![0,..]);

        // --- Step 4: Sum the Embeddings ---
        // The three embeddings are added together element-wise.
        // Broadcasting rules in ndarray handle adding the 1D arrays to the 2D array.
        let mut combined_embeddings = token_embeds;
        combined_embeddings += &position_embeds;
        combined_embeddings += &token_type_embeds;

        // --- Step 5: Apply Layer Normalization ---
        let normalized_embeddings = self.layer_norm(&combined_embeddings);

        normalized_embeddings
    }

    /// Applies Layer Normalization to the combined embeddings.
    /// This stabilizes the network and improves performance.
    fn layer_norm(&self, x: &Array2<f32>) -> Array2<f32> {
        let mean = x.mean_axis(Axis(1)).unwrap().insert_axis(Axis(1));
        let variance = x.var_axis(Axis(1), 0.0).insert_axis(Axis(1));
        let epsilon = 1e-12;

        let normalized = (x - &mean) / (variance + epsilon).mapv(f32::sqrt);

        // Apply the learned scale (gamma) and shift (beta) parameters.
        &self.layer_norm_gamma * normalized + &self.layer_norm_beta
    }
}