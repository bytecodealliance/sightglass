cmake_minimum_required(VERSION 3.27)

# Builds StarlingMonkey's `starling-raw.wasm` with the `bench.cpp` builtin added.
# Expects `STARLINGMONKEY_DIR` (a StarlingMonkey checkout) and `HOST_API` (this
# directory's `host-api/`) in the environment.
include("$ENV{STARLINGMONKEY_DIR}/cmake/add_as_subproject.cmake")

add_builtin(
    sightglass::bench
    SRC
        bench.cpp
    INCLUDE_DIRS
        ${CMAKE_CURRENT_SOURCE_DIR}/host-api/bindings)

project(sightglass-starlingmonkey)
