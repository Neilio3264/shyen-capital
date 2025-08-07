#include "market_data.hpp"
#include <fstream>
#include <sstream>
#include <iostream>

class CsvMarketData : public MarketData {
public:
    bool loadHistorical(const std::string& csvPath) override {
        std::ifstream in(csvPath);
        if (!in.is_open()) return false;

        bars_.clear();
        std::string line;
        // assume header: time,open,high,low,close,volume
        std::getline(in, line);
        while (std::getline(in, line)) {
            std::istringstream ss(line);
            std::string tok;
            Bar bar;
            // parse timestamp as seconds since epoch
            std::getline(ss, tok, ',');
            bar.timestamp = 
              std::chrono::system_clock::from_time_t(std::stoll(tok));
            std::getline(ss, tok, ','); bar.open = std::stod(tok);
            std::getline(ss, tok, ','); bar.high = std::stod(tok);
            std::getline(ss, tok, ','); bar.low  = std::stod(tok);
            std::getline(ss, tok, ','); bar.close= std::stod(tok);
            std::getline(ss, tok, ','); bar.volume = std::stod(tok);
            bars_.push_back(bar);
        }
        std::cout << "Loaded " << bars_.size() << " bars\n";
        return true;
    }

    const std::vector<Bar>& bars() const override {
        return bars_;
    }

    void startRealtime() override {
        // TODO: hook into TradingView websocket / API
        std::cout << "[Realtime] subscription started (stub)\n";
    }

private:
    std::vector<Bar> bars_;
};

// Factory function (optional)
std::unique_ptr<MarketData> makeCsvMarketData() {
    return std::make_unique<CsvMarketData>();
}
