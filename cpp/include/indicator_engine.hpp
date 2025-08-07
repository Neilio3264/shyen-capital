#pragma once
#include "market_data.hpp"
#include <vector>
#include <unordered_map>

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
