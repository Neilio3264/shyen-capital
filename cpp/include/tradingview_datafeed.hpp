// tradingview_datafeed.hpp
#pragma once
#include "market_data.hpp"

class TradingViewDatafeed : public MarketData {
public:
    TradingViewDatafeed(const std::string& sessionToken);
    bool loadHistorical(const std::string& symbol) override;
    const std::vector<Bar>& bars() const override { return bars_; }
    void startRealtime() override;
private:
    std::string sessionToken_;
    std::vector<Bar> bars_;
    void fetchHistory(const std::string& symbol, int from, int to);
    void connectWebSocket(const std::string& symbol);
};
