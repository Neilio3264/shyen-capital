#pragma once
#include "market_data.hpp"
#include <vector>
#include <unordered_map>

/// A struct to hold all computed indicator series
struct Indicators {
    std::vector<double> ema20;
    std::vector<double> ema50;
    std::vector<double> ema200;
    std::vector<double> rsi14;
    // (you can add more here: VWAP, ATR, etc.)
};

/// Compute the exponential moving average over `src` with look-back `period`.
/// Returns a vector the same length as `src`; 
/// for indices < period-1, the EMA is set to NaN.
std::vector<double> calcEMA(const std::vector<double>& src, int period);

/// Compute the RSI over `src` with look-back `period`.
/// Returns a vector the same length as `src`; for indices < period, RSI is set to NaN.
std::vector<double> calcRSI(const std::vector<double>& src, int period);

struct Indicators {
  std::vector<double> ema20;
  std::vector<double> ema50;
  std::vector<double> ema200;
  std::vector<double> rsi14;
  // add VWAP, ATR, etc.
};

class IndicatorEngine {
public:
  IndicatorEngine() = default;
  // Feed in the full bar history whenever it updates
  void compute(const std::vector<Bar>& bars);
  // Retrieve the latest computed indicators
  const Indicators& latest() const;
private:
  Indicators data_;
  // internal helpers: calcEMA(period), calcRSI(period), ...
};
