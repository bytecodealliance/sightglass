// A StarlingMonkey builtin that drives a benchmark.
//
// At build time, Wizer evaluates the benchmark's top-level script, which calls
// `registerBenchmark(inputPath, main)`. The snapshot keeps both, so at run time the
// `wasi:cli/run` export below only reads the input, calls `main(input)` between
// `bench.start` and `bench.end`, and prints the result.

#include "extension-api.h"
#include "benchmark.h"

#include <cstdio>
#include <string>

// Defined by wasi-libc but not declared in wasi-sdk 30's headers.
extern "C" void __wasilibc_reset_preopens(void);

namespace sightglass::bench {

static api::Engine *ENGINE = nullptr;
static JS::PersistentRootedObject MAIN;
static std::string INPUT_PATH;

static bool register_benchmark(JSContext *cx, unsigned argc, JS::Value *vp) {
  JS::CallArgs args = JS::CallArgsFromVp(argc, vp);
  if (!args.requireAtLeast(cx, "registerBenchmark", 2)) {
    return false;
  }
  if (!args[0].isString() || !args[1].isObject() || !JS::IsCallable(&args[1].toObject())) {
    return api::throw_error(cx, api::Errors::TypeError, "registerBenchmark", "arguments",
                            "be (string, function)");
  }
  JS::RootedString path(cx, args[0].toString());
  JS::UniqueChars path_chars = JS_EncodeStringToUTF8(cx, path);
  if (!path_chars) {
    return false;
  }
  INPUT_PATH = path_chars.get();
  MAIN.init(cx, &args[1].toObject());
  args.rval().setUndefined();
  return true;
}

static const JSFunctionSpec methods[] = {
    JS_FN("registerBenchmark", register_benchmark, 2, 0),
    JS_FS_END,
};

bool install(api::Engine *engine) {
  ENGINE = engine;
  return JS_DefineFunctions(engine->cx(), engine->global(), methods);
}

static bool read_file(const char *path, std::string &out) {
  FILE *f = fopen(path, "rb");
  if (!f) {
    return false;
  }
  char buf[64 * 1024];
  size_t n;
  while ((n = fread(buf, 1, sizeof(buf), f)) > 0) {
    out.append(buf, n);
  }
  fclose(f);
  return true;
}

} // namespace sightglass::bench

using namespace sightglass::bench;

extern "C" bool exports_benchmark_run_run() {
  if (!MAIN) {
    fprintf(stderr, "no benchmark registered during pre-initialization\n");
    return false;
  }

  // libc's preopen table was populated while Wizer loaded the script and still
  // describes the build-time directory; make it rediscover this run's preopens.
  __wasilibc_reset_preopens();
  std::string input;
  if (!read_file(INPUT_PATH.c_str(), input)) {
    fprintf(stderr, "failed to read %s\n", INPUT_PATH.c_str());
    return false;
  }

  JSContext *cx = ENGINE->cx();
  JSAutoRealm ar(cx, ENGINE->global());

  // Latin-1, like the `spidermonkey` benchmarks, so string lengths (and thus
  // stdout) match theirs byte for byte.
  JS::RootedString input_str(cx, JS_NewStringCopyN(cx, input.data(), input.size()));
  if (!input_str) {
    ENGINE->dump_pending_exception("creating the input string");
    return false;
  }
  JS::RootedValue arg(cx, JS::StringValue(input_str));
  JS::RootedValue fn(cx, JS::ObjectValue(*MAIN));
  JS::RootedValue result(cx);

  bench_start();
  bool ok = JS_CallFunctionValue(cx, ENGINE->global(), fn, JS::HandleValueArray(arg), &result);
  bench_end();

  if (!ok) {
    ENGINE->dump_pending_exception("running main");
    return false;
  }
  JS::RootedString result_str(cx, JS::ToString(cx, result));
  if (!result_str) {
    return false;
  }
  JS::UniqueChars out = JS_EncodeStringToLatin1(cx, result_str);
  if (!out) {
    return false;
  }
  fputs(out.get(), stdout);
  fputs("All done!\n", stdout);
  fflush(stdout);
  return true;
}
