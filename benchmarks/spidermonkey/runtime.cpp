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

#include "marked_js.h"
#include "main_js.h"

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

bool eval_bench(JSContext* cx,
                JS::HandleObject global,
                const char* markedSrc,
                size_t markedSrcLen,
                const char* mainSrc,
                size_t mainSrcLen,
                const char* markdownInput) {
  JS::SourceText<mozilla::Utf8Unit> markedSrcBuf;
  if (!markedSrcBuf.init(cx, markedSrc, markedSrcLen, JS::SourceOwnership::Borrowed)) {
    return false;
  }

  JS::SourceText<mozilla::Utf8Unit> mainSrcBuf;
  if (!mainSrcBuf.init(cx, mainSrc, mainSrcLen, JS::SourceOwnership::Borrowed)) {
    return false;
  }

  // Compile and execute the top level of `marked.min.js`.
  //
  // Note that we do this outside of `bench_{start,end}` because this stuff will
  // typically get Wizer'd away from what is the actual hot path for JS
  // execution.
  JS::CompileOptions markedOpts(cx);
  markedOpts.setForceFullParse();
  markedOpts.setFileAndLine("marked.min.js", 1);
  JS::RootedScript markedScript(cx, JS::Compile(cx, markedOpts, markedSrcBuf));
  if (!markedScript) {
    return false;
  }
  JS::RootedValue result(cx);
  if (!JS_ExecuteScript(cx, markedScript, &result)) {
    return false;
  }

  // Similarly for `main.js`.
  JS::CompileOptions mainOpts(cx);
  mainOpts.setForceFullParse();
  mainOpts.setFileAndLine("main.js", 1);
  JS::RootedScript mainScript(cx, JS::Compile(cx, mainOpts, mainSrcBuf));
  if (!mainScript) {
    return false;
  }
  if (!JS_ExecuteScript(cx, mainScript, &result)) {
    return false;
  }

  JS::RootedValue arg(cx, JS::StringValue(JS_NewStringCopyZ(cx, markdownInput)));

  // Run `main(markdownInput)`. Do this within `bench_{start,end}` since this
  // would be the actual JS hot path after Wizening.
  bench_start();
  if (!JS_CallFunctionName(cx, global, "main", JS::HandleValueArray(arg), &result)) {
    return false;
  }
  bench_end();

  JS::RootedString resultStr(cx, result.toString());
  JS::AutoAssertNoGC nogc(cx);
  size_t len = 0;
  const JS::Latin1Char* chars = JS_GetLatin1StringCharsAndLength(cx, nogc, resultStr, &len);
  printf("%.*s", (int) len, chars);

  return true;
}

char* readFile(const char* path) {
  FILE* f = fopen(path, "r");
  assert(f && "should open path okay");

  fseek(f, 0, SEEK_END);
  ssize_t len = ftell(f);
  fseek(f, 0, SEEK_SET);

  char* contents = (char*) malloc(len + 1);
  assert(contents && "should malloc contents okay");

  ssize_t nread = fread(contents, len, 1, f);
  assert(nread == 1 && "should read all bytes");

  fclose(f);
  contents[len] = '\0';
  return contents;
}

int main(int argc, const char *argv[]) {

  if (!init_js()) {
    exit(1);
  }

  JSContext* cx = CONTEXT;
  RootedObject global(cx, GLOBAL);
  JSAutoRealm ar(cx, global);
  FETCH_HANDLERS = new JS::PersistentRootedObjectVector(cx);

  char* markdownInput = readFile("default.input.md");

  if (!eval_bench(cx,
                  global,
                  (const char*) js_marked_min_js,
                  js_marked_min_js_len,
                  (const char*) js_main_js,
                  js_main_js_len,
                  markdownInput)) {
    fprintf(stderr, "Error evaluating JS!\n");
    JS::RootedValue exn(cx);
    if (JS_GetPendingException(cx, &exn)) {
      JS::RootedObject exnObj(cx, &exn.toObject());
      JSErrorReport* report = JS_ErrorFromException(cx, exnObj);
      JS::PrintError(stderr, report, true);
    }
    exit(1);
  }

  printf("All done!\n");
}
