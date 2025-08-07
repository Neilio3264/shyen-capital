cmake_minimum_required(VERSION 3.16)
project(shyen_capital VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_BUILD_TYPE Release)

# Main library target
add_library(shyen_core
    src/market_data.cpp
    src/indicator_engine.cpp
)
target_include_directories(shyen_core PUBLIC include)

# Executable
add_executable(shyen_bot src/main.cpp)
target_link_libraries(shyen_bot PRIVATE shyen_core)

# Tests
add_executable(test_indicator tests/test_indicator_engine.cpp src/indicator_engine.cpp)
target_link_libraries(test_indicator PRIVATE shyen_core)