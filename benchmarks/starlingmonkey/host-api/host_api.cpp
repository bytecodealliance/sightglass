// The subset of StarlingMonkey's host API that its runtime and non-HTTP builtins need, implemented
// on the `wit-bindgen` bindings for `wit/world.wit`. Adapted from StarlingMonkey's
// `host-apis/wasi-0.2.0/host_api.cpp`.

#include "host_api.h"
#include "benchmark.h"

#include <algorithm>

static std::optional<wasi_clocks_monotonic_clock_own_pollable_t> immediately_ready;

static size_t poll_handles(std::vector<wasi_io_poll_borrow_pollable_t> handles) {
  auto list = wasi_io_poll_list_borrow_pollable_t{handles.data(), handles.size()};
  benchmark_list_u32_t result{nullptr, 0};
  wasi_io_poll_poll(&list, &result);
  MOZ_ASSERT(result.len > 0);
  auto ready_index = *std::min_element(result.ptr, result.ptr + result.len);
  free(result.ptr);
  return ready_index;
}

size_t api::AsyncTask::select(std::vector<RefPtr<AsyncTask>> &tasks) {
  std::erase_if(tasks, [](const RefPtr<AsyncTask> &task) {
    return task->id() == INVALID_POLLABLE_HANDLE;
  });

  std::vector<wasi_io_poll_borrow_pollable_t> handles;
  for (size_t idx = 0; idx < tasks.size(); ++idx) {
    auto id = tasks.at(idx)->id();
    if (id == IMMEDIATE_TASK_HANDLE) {
      if (handles.empty()) {
        return idx;
      }
      if (!immediately_ready) {
        immediately_ready = wasi_clocks_monotonic_clock_subscribe_duration(0);
      }
      handles.push_back({immediately_ready->__handle});
      size_t len = handles.size();
      size_t ready_index = poll_handles(std::move(handles));
      return ready_index <= len - 1 ? ready_index : idx;
    }
    handles.push_back({id});
  }
  return poll_handles(std::move(handles));
}

namespace host_api {

Result<HostBytes> Random::get_bytes(size_t num_bytes) {
  benchmark_list_u8_t list{};
  wasi_random_random_get_random_bytes(num_bytes, &list);
  return Result<HostBytes>::ok(HostBytes{std::unique_ptr<uint8_t[]>{list.ptr}, list.len});
}

Result<uint32_t> Random::get_u32() {
  return Result<uint32_t>::ok(wasi_random_random_get_random_u64());
}

uint64_t MonotonicClock::now() { return wasi_clocks_monotonic_clock_now(); }

uint64_t MonotonicClock::resolution() { return wasi_clocks_monotonic_clock_resolution(); }

int32_t MonotonicClock::subscribe(const uint64_t when, const bool absolute) {
  return absolute ? wasi_clocks_monotonic_clock_subscribe_instant(when).__handle
                  : wasi_clocks_monotonic_clock_subscribe_duration(when).__handle;
}

void MonotonicClock::unsubscribe(const int32_t handle_id) {
  wasi_io_poll_pollable_drop_own({handle_id});
}

vector<std::string> environment_get_arguments() {
  benchmark_list_string_t raw_args = {};
  wasi_cli_environment_get_arguments(&raw_args);
  std::vector<std::string> args;
  for (size_t i = 0; i < raw_args.len; i++) {
    args.emplace_back(reinterpret_cast<char *>(raw_args.ptr[i].ptr), raw_args.ptr[i].len);
  }
  return args;
}

void handle_api_error(JSContext *cx, uint8_t err, int line, const char *func) {
  JS_ReportErrorUTF8(cx, "%s: An error occurred while using the host API.\n", func);
}

} // namespace host_api
