// /shyen_capital/crates/sc_nlp/src/model/mod.rs

/// This module contains the building blocks of the Shyen Capital neural network model.
/// Each sub-module represents a key component of the Transformer architecture,
/// allowing for a clean, modular, and maintainable design.

// The embedding layer is responsible for converting token IDs into vectors.
pub mod embedding;

// The attention module contains the multi-head self-attention mechanism.
// pub mod attention; // We will build this next.

// The encoder module combines the attention and feed-forward layers.
// pub mod encoder; // We will build this after attention.