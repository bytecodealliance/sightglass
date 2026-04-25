# sqlite3

A WebAssembly build of SQLite's official `speedtest1.c` benchmark program,
ported from [JetStream 3](https://github.com/WebKit/JetStream/tree/main/sqlite3)
for use with Sightglass.

Quoting from [its description](https://sqlite.org/cpu.html):

> This program strives to exercise the SQLite library in a way that is typical
> of real-world applications. Of course, every application is different, and so
> no test program can exactly mirror the behavior of all applications.

Since SQLite is a very widely used database and provides an official and popular
upstream WebAssembly port, this is a realistic, larger WebAssembly program.

Originally built from SQLite 3.48.0 with Emscripten SDK 3.1.73.

## Porting notes

The original JetStream 3 module was compiled by Emscripten and paired with a
JavaScript driver. Since sightglass does not use JavaScript, the module was
modified to:

- Remove Emscripten `"env"` imports and replace them with stubs (except
  `emscripten_resize_heap`, which delegates to `memory.grow`).
- Add `"bench" "start"` and `"bench" "end"` imports for Sightglass timing.
- Add a `"_start"` export that calls `__wasm_call_ctors`, `bench.start`,
  `wasm_main`, and `bench.end`.
- Execute the sqlite3 benchmarks with `szTest=25` instead of `szTest=100`,
  bringing a default run under Sightglass from ~4 minutes to ~1 minute.

## License

The SQLite source code is public domain:

> The author disclaims copyright to this source code. In place of a legal
> notice, here is a blessing:
>
> - May you do good and not evil.
> - May you find forgiveness for yourself and forgive others.
> - May you share freely, never taking more than you give.
