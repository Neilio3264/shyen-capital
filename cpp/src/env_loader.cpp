#include "env_loader.hpp"
#include <fstream>
#include <sstream>
#include <cstdlib>

bool loadDotEnv(const std::string& filePath) {
    std::ifstream in(filePath);
    if (!in.is_open()) return false;
    std::string line;
    while (std::getline(in, line)) {
        // Skip empty/comment lines
        if (line.empty() || line[0] == '#') continue;
        auto eq = line.find('=');
        if (eq == std::string::npos) continue;
        std::string key   = line.substr(0, eq);
        std::string value = line.substr(eq + 1);
        // Trim whitespace (simple)
        key.erase(0, key.find_first_not_of(" \t\r"));
        key.erase(key.find_last_not_of(" \t\r") + 1);
        value.erase(0, value.find_first_not_of(" \t\r"));
        value.erase(value.find_last_not_of(" \t\r") + 1);
        // Set into environment (overwrite existing)
        ::setenv(key.c_str(), value.c_str(), /*overwrite=*/1);
    }
    return true;
}
