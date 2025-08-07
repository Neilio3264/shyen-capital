// /shyen_capital/crates/sc_core/src/lib.rs

use serde::{Deserialize, Serialize};

/// Represents a single, standardized market data event.
/// This struct is the lifeblood of the quantitative side of our engine
/// `#[repr(C)]` ensures that the memory layout is compatible with C,
/// which is a best practice for high-performance and interoperable systems.
#[repr(C)]
#
pub struct MarketTick {
    /// The stock symbol, represented as a fixed-size byte array for performance.
    pub symbol: [u8; 16],
    /// The price of the asset at the time of the tick.
    pub price: f64,
    /// The volume of the asset traded at that moment.
    pub volume: u64,
    /// UNIX timestamp with nanosecond precision for high-frequency analysis.
    pub timestamp_ns: u128,
}

/// Represents a single, standardized sentiment data point from any source.
/// This struct is the core of our narrative analysis.
#[repr(C)]
#
pub struct SentimentDataPoint {
    /// The source of the sentiment (0: X/Twitter, 1: Reddit, 2: News, etc.).
    /// Using a u8 is more memory-efficient than a string.
    pub source: u8,
    /// The calculated sentiment score, normalized from -1.0 (negative) to 1.0 (positive).
    pub score: f32,
    /// UNIX timestamp with nanosecond precision to correlate with market ticks.
    pub timestamp_ns: u128,
}

/// A unified data packet that can contain either a market tick or a sentiment point.
/// This allows us to process all incoming data in a single, time-ordered stream.
#
pub enum DataPacket {
    Market(MarketTick),
    Sentiment(SentimentDataPoint),
}pub struct MarketTick {
    /// The stock symb
}