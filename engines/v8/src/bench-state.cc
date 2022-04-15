#include "bench-state.hh"
#include <iostream>

Config Config::make(wasm_bench_config_t config) {
  auto working_dir =
      std::string(config.working_dir_ptr, config.working_dir_len);
  auto stdout_path =
      std::string(config.stdout_path_ptr, config.stdout_path_len);
  auto stderr_path =
      std::string(config.stderr_path_ptr, config.stderr_path_len);
  auto stdin_path = std::string(config.stdin_path_ptr, config.stdin_path_len);
  auto compilation = Timer(config.compilation_timer, config.compilation_start,
                           config.compilation_end);
  auto instantiation = Timer(config.compilation_timer, config.compilation_start,
                             config.compilation_end);
  auto execution = Timer(config.compilation_timer, config.compilation_start,
                         config.compilation_end);
  return Config(working_dir, stdout_path, stderr_path, stdin_path, compilation,
                instantiation, execution);
}

void Timer::start() const {
  if (this->start_timer != NULL) {
    (this->start_timer)(this->timer);
  }
}

void Timer::end() const {
  if (this->end_timer != NULL) {
    (this->end_timer)(this->timer);
  }
}
