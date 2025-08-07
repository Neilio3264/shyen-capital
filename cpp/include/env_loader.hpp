#pragma once
#include <string>

/// Load KEY=VALUE lines from `.env` into the process environment
/// Returns true if file was parsed successfully.
bool loadDotEnv(const std::string& filePath);
