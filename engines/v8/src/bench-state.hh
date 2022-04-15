// Data structures maintained during a Sightglass benchmarking session.
#ifndef __BENCH_STATE_HH
#define __BENCH_STATE_HH

#include <optional>
#include "bench-api.h"
#include "wasm.hh"

/**
 * @brief A C++-wrapper around the timer pointers passed in `config_t`.
 */
class Timer {
 public:
  Timer(wasm_bench_timer_t timer,
        wasm_bench_callback_t start_timer,
        wasm_bench_callback_t end_timer)
      : timer(timer), start_timer(start_timer), end_timer(end_timer) {}
  void start() const;
  void end() const;

 private:
  wasm_bench_timer_t timer;
  wasm_bench_callback_t start_timer;
  wasm_bench_callback_t end_timer;
};

/**
 * @brief A C++-wrapper around the `config_t` passed in `wasm_bench_create`.
 */
class Config {
 public:
  static Config make(wasm_bench_config_t config);
  std::string working_dir;
  std::string stdout_path;
  std::string stderr_path;
  std::string stdin_path;
  Timer compilation;
  Timer instantiation;
  Timer execution;

 private:
  Config(std::string working_dir,
         std::string stdout_path,
         std::string stderr_path,
         std::string stdin_path,
         Timer compilation,
         Timer instantiation,
         Timer execution)
      : working_dir(working_dir),
        stdout_path(stdout_path),
        stderr_path(stderr_path),
        stdin_path(stdin_path),
        compilation(compilation),
        instantiation(instantiation),
        execution(execution) {}
};

/**
 * @brief The internal state maintained through a Sightglass benchmarking
 * session.
 */
class BenchState {
 public:
  BenchState(Config config) : config(config) {}
  Config config;
  std::unique_ptr<wasm::Engine> engine;
  std::unique_ptr<wasm::Store> store;
  std::unique_ptr<wasm::Module> module;
  std::unique_ptr<wasm::Instance> instance;
  // std::optional<wasm::Store> store;
  // std::optional<wasm::Module> wasm_module;
  // std::optional<wasm::Instance> instance;
};

#endif  // #ifdef __BENCH_STATE_HH
