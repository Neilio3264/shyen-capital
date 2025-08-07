#pragma once
#include "indicator_engine.hpp"
#include <chrono>

enum class Trend { Up, Down, Flat };

class TrendPredictor {
public:
  TrendPredictor(double fullAcc, double dirtyAcc);
  // Call whenever new indicators are ready; returns full (heavy) prediction
  Trend predictFull(const Indicators& ind);
  // Call on every new bar for a quick verdict
  Trend predictDirty(const Indicators& ind);
};
