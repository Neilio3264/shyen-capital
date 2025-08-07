#include "env_loader.hpp"
#include "tradingview_datafeed.hpp"
#include "indicator_engine.hpp"
#include "trend_predictor.hpp"
#include "options_selector.hpp"
#include <iostream>
#include <cstdlib>

int main(int argc, char* argv[]) {
    // 1️⃣ Load .env before anything else
    if (!loadDotEnv(".env")) {
        std::cerr << "Warning: .env file not found or failed to parse\n";
    }

    // 2️⃣ Read the session token from env
    const char* tokenCStr = std::getenv("TV_SESSION_TOKEN");
    if (!tokenCStr) {
        std::cerr << "Error: TV_SESSION_TOKEN not set in environment\n";
        return 1;
    }
    std::string sessionToken(tokenCStr);

    // 3️⃣ Construct your datafeed with the masked token
    auto md = std::make_unique<TradingViewDatafeed>(sessionToken);

    // 4️⃣ Proceed as normal
    if (!md->loadHistorical("SPY")) {
        std::cerr << "Failed to fetch historical data\n";
        return 1;
    }
    std::cout << "Loaded " << md->bars().size() << " bars\n";

    IndicatorEngine ie;
    TrendPredictor tp(0.95, 0.75);
    OptionsSelector os;

    md->startRealtime();
    while (true) {
    auto bars = md->bars();
    ie.compute(bars);
    auto ind = ie.latest();
    auto trend = tp.predictDirty(ind);
    // every 15m: tp.predictFull(ind);
    auto signals = os.scan(trend);
    // dispatch alerts…
    }

    return 0;
}
