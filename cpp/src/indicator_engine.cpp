#include "indicator_engine.hpp"
#include <stdexcept>

// Simple EMA helper (exponential smoothing)
static std::vector<double> calcEMA(const std::vector<double>& src, int period);

void IndicatorEngine::compute(const std::vector<Bar>& bars) {
  if (bars.empty()) throw std::runtime_error("No bars for indicators");
  std::vector<double> closes;
  closes.reserve(bars.size());
  for (auto& b : bars) closes.push_back(b.close);

  data_.ema20  = calcEMA(closes, 20);
  data_.ema50  = calcEMA(closes, 50);
  data_.ema200 = calcEMA(closes, 200);
  // calcRSI, VWAP, etc. in the same fashion
}

const Indicators& IndicatorEngine::latest() const {
  return data_;
}
