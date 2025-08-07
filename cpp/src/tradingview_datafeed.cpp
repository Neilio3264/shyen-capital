// tradingview_datafeed.cpp
#include "tradingview_datafeed.hpp"
#include <curl/curl.h>
#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>
#include <nlohmann/json.hpp>

// ... in constructor capture sessionToken for headers
bool TradingViewDatafeed::loadHistorical(const std::string& symbol) {
    int to   = std::time(nullptr);
    int from = to - 3600*24*365*12; // 12 years back
    fetchHistory(symbol, from, to);
    return !bars_.empty();
}

void TradingViewDatafeed::fetchHistory(const std::string& symbol,
                                       int from, int to) {
    // ❗️ 1. Build URL
    //    https://api.tradingview.com/history?symbol=SPY&resolution=1&from=...&to=...
    std::ostringstream url;
    url << "https://api.tradingview.com/history"
        << "?symbol="   << symbol
        << "&resolution=1"
        << "&from="     << from
        << "&to="       << to;

    // ❗️ 2. Setup libcurl
    CURL* curl = curl_easy_init();
    if (!curl) throw std::runtime_error("Failed to init curl");

    std::string response;
    curl_easy_setopt(curl, CURLOPT_URL, url.str().c_str());
    // Callback to collect the body
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, 
        +[](char* ptr, size_t size, size_t nmemb, void* userdata) {
            auto& buf = *static_cast<std::string*>(userdata);
            buf.append(ptr, size*nmemb);
            return size*nmemb;
        });
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);
    // Pass your session token via header
    struct curl_slist* headers = nullptr;
    headers = curl_slist_append(headers,
        ("Cookie: session_id=" + sessionToken_).c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

    // ❗️ 3. Perform & cleanup
    CURLcode res = curl_easy_perform(curl);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    if (res != CURLE_OK)
        throw std::runtime_error("curl error: " + std::string(curl_easy_strerror(res)));

    // ❗️ 4. Parse JSON response into Bar structs
    auto json = nlohmann::json::parse(response);
    auto& t = json["t"];
    auto& o = json["o"];
    // ... similarly h, l, c, v

    bars_.clear();
    for (size_t i = 0; i < t.size(); ++i) {
        Bar bar;
        bar.timestamp = 
          std::chrono::system_clock::from_time_t(t[i].get<long>());
        bar.open      = o[i].get<double>();
        bar.high      = json["h"][i].get<double>();
        bar.low       = json["l"][i].get<double>();
        bar.close     = json["c"][i].get<double>();
        bar.volume    = json["v"][i].get<double>();
        bars_.push_back(bar);
    }
}


void TradingViewDatafeed::startRealtime() {
    connectWebSocket("SPY");
    // send subscribeBars message, parse incoming, append to bars_
}

void TradingViewDatafeed::connectWebSocket(const std::string& symbol) {
    using ws_client = websocketpp::client<websocketpp::config::asio_client>;
    ws_client client;

    client.init_asio();
    client.set_message_handler(
      [&](websocketpp::connection_hdl, ws_client::message_ptr msg) {
        // Parse incoming JSON into Bar
        auto j = nlohmann::json::parse(msg->get_payload());
        // …extract t,o,h,l,c,v as before…
        Bar bar{/*...*/};
        bars_.push_back(bar);
      });

    // 1️⃣ Build URI with token
    std::string uri = "wss://stream.tradingview.com/symbols"
                      "?session_id=" + sessionToken_;

    websocketpp::uri uri_obj(uri);
    auto con = client.get_connection(uri, websocketpp::lib::error_code());
    
    client.connect(con);
    client.run_one();  // process handshake

    // 2️⃣ Subscribe to bar updates
    nlohmann::json subMsg = {
      { "method", "subscribeBars" },
      { "params", {
          { "symbol", symbol },
          { "resolution", "1" }
        }
      }
    };
    con->send(subMsg.dump());

    // 3️⃣ Hand off to ASIO loop for continuous receive
    client.start_perpetual();
    std::thread([&](){ client.run(); }).detach();
}
