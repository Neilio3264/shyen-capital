#pragma once
#include <vector>
#include <string>
#include <chrono>

// A single OHLCV bar
struct Bar {
    std::chrono::system_clock::time_point timestamp;
    double open, high, low, close;
    double volume;
};

// Interface for loading historical data and subscribing real-time
class MarketData {
public:
    virtual ~MarketData() = default;

    // Load from CSV (for backtesting)
    virtual bool loadHistorical(const std::string& csvPath) = 0;

    // Get all loaded bars
    virtual const std::vector<Bar>& bars() const = 0;

    // Stub: subscribe to real-time feed
    virtual void startRealtime() = 0;
};
