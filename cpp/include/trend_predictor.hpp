#pragma once
#include "indicator_engine.hpp"
#include <chrono>

enum class Trend { Up, Down, Flat };

/// A quick “dirty” verdict in ~2 s using only the last bar
Trend predictDirty(const Indicators& ind);

/// A “full” 15 min verdict using the last K bars’ worth of features
Trend predictFull(const std::vector<Indicators>& history);

class TrendPredictor {
public:
  TrendPredictor(double fullAcc, double dirtyAcc);
  // Call whenever new indicators are ready; returns full (heavy) prediction
  Trend predictFull(const Indicators& ind);
  // Call on every new bar for a quick verdict
  Trend predictDirty(const Indicators& ind);
};
