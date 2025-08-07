#pragma once
#include <vector>
#include <string>

struct OptionSignal {
  std::string symbol;  // e.g. "SPY_20250804_631C"
  double price;
  double impliedVol;
  double expectedMove; // % move needed
  double stopLoss;     // price
  double takeProfit;   // price
};

class OptionsSelector {
public:
  // Call with current trend & real-time quotes; returns candidates
  std::vector<OptionSignal> scan(Trend trend);
};
