#include "trend_predictor.hpp"

// Dirty: look at just the last two EMA20 values
Trend predictDirty(const Indicators& ind) {
  int n = ind.ema20.size();
  if (n < 2) return Trend::Flat;
  double delta = ind.ema20[n-1] - ind.ema20[n-2];
  if (delta > 0) return Trend::Up;
  if (delta < 0) return Trend::Down;
  return Trend::Flat;
}

// Full: X-bar linear regression on EMA50 slope + RSI momentum
Trend predictFull(const std::vector<Indicators>& hist) {
  if (hist.size() < 15) return Trend::Flat;
  // compute slope of ema50 over last 15 points
  double sumX=0, sumY=0, sumXY=0, sumX2=0;
  int N = 15;
  for (int i=0; i<N; ++i) {
    double x = i;
    double y = hist[hist.size()-N + i].ema50.back();
    sumX  += x;
    sumY  += y;
    sumXY += x*y;
    sumX2 += x*x;
  }
  double slope = (N*sumXY - sumX*sumY)/(N*sumX2 - sumX*sumX);
  // RSI change over last window
  auto& rsi = hist.back().rsi14;
  double rsiDelta = rsi.back() - hist[hist.size()-N].rsi14.front();

  // simple rules
  if (slope > 0 && rsiDelta > 0) return Trend::Up;
  if (slope < 0 && rsiDelta < 0) return Trend::Down;
  return Trend::Flat;
}
