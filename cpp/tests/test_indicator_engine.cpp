#include "indicator_engine.hpp"
#include <cassert>
using namespace std;

int main() {
  // 5-bar series: [1, 2, 3, 4, 5]
  vector<double> v{1,2,3,4,5};
  auto ema3 = calcEMA(v, 3);
  // SMA(1,2,3)=2 → ema[2]=2; ema[3]=3*0.5 + 2*0.5 = 2.5, etc.
  assert(fabs(ema3[2] - 2.0) < 1e-9);
  assert(fabs(ema3[3] - 2.5) < 1e-9);
  assert(fabs(ema3[4] - 3.25) < 1e-9);

  auto rsi2 = calcRSI(v, 2);
  // deltas: [+1,+1,+1,+1], avgGain(0→2)=1, avgLoss=0 → RSI[2]=100
  assert(fabs(rsi2[2] - 100.0) < 1e-9);
  return 0;
}
