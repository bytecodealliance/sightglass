// Largely borrowed barebones SpiderMonkey toplevel wrapper from:
// https://github.com/fastly/js-compute-runtime/blob/main/c-dependencies/js-compute-runtime/js-compute-runtime.cpp

#include <cassert>
#include <cstdlib>
#include <iostream>

#include <wasi/libc-environ.h>

#include "sightglass.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Winvalid-offsetof"

#include "jsapi.h"
#include "jsfriendapi.h"

#include "js/CompilationAndEvaluation.h"
#include "js/ContextOptions.h"
#include "js/ForOfIterator.h"
#include "js/Initialization.h"
#include "js/Object.h"
#include "js/SourceText.h"

#pragma clang diagnostic pop

using JS::Value;

using JS::RootedValue;
using JS::RootedObject;
using JS::RootedString;

using JS::HandleValue;
using JS::HandleObject;
using JS::MutableHandleValue;

using JS::PersistentRooted;
using JS::PersistentRootedVector;

/* The class of the global object. */
static JSClass global_class = {
    "global",
    JSCLASS_GLOBAL_FLAGS,
    &JS::DefaultGlobalClassOps
};

JSContext* CONTEXT = nullptr;

JS::PersistentRootedObject GLOBAL;

static JS::PersistentRootedObjectVector* FETCH_HANDLERS;

bool init_js() {
  JS_Init();

  JSContext *cx = JS_NewContext(JS::DefaultHeapMaxBytes);
  if (!cx)
      return false;
  if (!js::UseInternalJobQueues(cx) || !JS::InitSelfHostedCode(cx))
      return false;

  JS::ContextOptionsRef(cx)
    .setPrivateClassFields(true)
    .setPrivateClassMethods(true)
    .setClassStaticBlocks(true)
    .setErgnomicBrandChecks(true);

  JS::RealmOptions options;
  options.creationOptions()
    .setStreamsEnabled(true)
    .setReadableByteStreamsEnabled(true)
    .setBYOBStreamReadersEnabled(true)
    .setReadableStreamPipeToEnabled(true)
    .setWritableStreamsEnabled(true)
    .setIteratorHelpersEnabled(true)
    .setWeakRefsEnabled(JS::WeakRefSpecifier::EnabledWithoutCleanupSome);

  RootedObject global(cx, JS_NewGlobalObject(cx, &global_class, nullptr, JS::FireOnNewGlobalHook,
                                             options));
  if (!global)
      return false;

  JSAutoRealm ar(cx, global);
  if (!JS::InitRealmStandardClasses(cx))
    return false;


  CONTEXT = cx;
  GLOBAL.init(cx, global);

  return true;
}

static const char* BENCH =
  "function fib(n) {\n"
  "  if (n < 2) {\n"
  "    return 1;\n"
  "  } else {\n"
  "    return fib(n-1) + fib(n-2);\n"
  "  }\n"
  "}\n"
  "\n"
  "fib(33);\n";

bool eval_bench(JSContext* cx, MutableHandleValue result) {
  JS::CompileOptions opts(cx);
  opts.setForceFullParse();
  opts.setFileAndLine("<stdin>", 1);

  JS::SourceText<mozilla::Utf8Unit> srcBuf;
  if (!srcBuf.init(cx, BENCH, strlen(BENCH), JS::SourceOwnership::TakeOwnership)) {
      return false;
  }

  JS::RootedScript script(cx);
  script = JS::Compile(cx, opts, srcBuf);
  if (!script) return false;

  bench_start();

  // Execute the top-level script.
  if (!JS_ExecuteScript(cx, script, result))
    return false;

  bench_end();

  return true;
}

int main(int argc, const char *argv[]) {
  if (!init_js())
    exit(1);

  JSContext* cx = CONTEXT;
  RootedObject global(cx, GLOBAL);
  JSAutoRealm ar(cx, global);
  FETCH_HANDLERS = new JS::PersistentRootedObjectVector(cx);

  RootedValue result(cx);
  if (!eval_bench(cx, &result)) {
    fprintf(stderr, "Error evaluating JS\n");
    exit(1);
  }
}
