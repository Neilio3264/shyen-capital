#include "indicator_engine.hpp"
#include <cmath>
#include <stdexcept>

// Helper to compute the simple moving average of the first 'period' values.
static double simpleMA(const std::vector<double>& src, int period) {
    double sum = 0.0;
    for (int i = 0; i < period; ++i) sum += src[i];
    return sum / period;
}

std::vector<double> calcEMA(const std::vector<double>& src, int period) {
    int n = src.size();
    if (period < 1 || period > n)
        throw std::invalid_argument("EMA period out of range");

    std::vector<double> ema(n, std::nan("")); 
    double alpha = 2.0 / (period + 1.0);

    // 1️⃣ Seed EMA at index period-1 with SMA
    ema[period - 1] = simpleMA(src, period);

    // 2️⃣ Iterate forward applying EMA formula
    for (int t = period; t < n; ++t) {
        ema[t] = src[t] * alpha 
               + ema[t - 1] * (1.0 - alpha);
    }

    return ema;
}

std::vector<double> calcRSI(const std::vector<double>& src, int period) {
    int n = src.size();
    if (period < 1 || period >= n)
        throw std::invalid_argument("RSI period out of range");

    std::vector<double> rsi(n, std::nan(""));

    // 1️⃣ Compute first N gains and losses
    double sumGain = 0.0, sumLoss = 0.0;
    for (int i = 1; i <= period; ++i) {
        double delta = src[i] - src[i-1];
        if (delta > 0) sumGain += delta;
        else           sumLoss += -delta;
    }
    double avgGain = sumGain / period;
    double avgLoss = sumLoss / period;

    // 2️⃣ RSI is undefined for indices < period
    //    RSI at t = period is calculable
    double rs    = avgLoss == 0 ? 0 : avgGain / avgLoss;
    rsi[period]  = 100.0 - (100.0 / (1.0 + rs));

    // 3️⃣ Wilder’s smoothing for the rest
    for (int t = period + 1; t < n; ++t) {
        double delta = src[t] - src[t-1];
        double gain  = delta > 0 ? delta : 0;
        double loss  = delta < 0 ? -delta : 0;

        avgGain = (avgGain * (period - 1) + gain) / period;
        avgLoss = (avgLoss * (period - 1) + loss) / period;

        rs        = avgLoss == 0 ? 0 : avgGain / avgLoss;
        rsi[t]    = 100.0 - (100.0 / (1.0 + rs));
    }

    return rsi;
}

void IndicatorEngine::compute(const std::vector<Bar>& bars) {
  if (bars.empty()) throw std::runtime_error("No bars for indicators");
  std::vector<double> closes;
  closes.reserve(bars.size());
  for (auto& b : bars) closes.push_back(b.close);

  data_.ema20  = calcEMA(closes, 20);
  data_.ema50  = calcEMA(closes, 50);
  data_.ema200 = calcEMA(closes, 200);
  data_.rsi14  = calcRSI(closes,  14);

  // … you can slot in VWAP, ATR, etc., here …
}

const Indicators& IndicatorEngine::latest() const {
  return data_;
}
