#!/usr/bin/env bash
# Builds `dart-deltablue.wasm` from `src/`. Run by the Dockerfile; expects
# `dart`, `wasm-merge`, `wasm-opt` and `wasm-tools` on PATH and the preview1
# command adapter at $ADAPTER.
set -euo pipefail

SRC=$(cd "$(dirname "$0")" && pwd)/src
OUT=${1:-dart-deltablue.wasm}
WORK=$(mktemp -d)

# The standalone target needs no JS; experimental FFI lets `main.dart` import
# WASI and `bench` functions directly.
dart compile wasm --standalone -O4 -E --enable-experimental-ffi \
  --no-source-maps -o "$WORK/dart.wasm" "$SRC/main.dart"

# Link in the `dart.*` embedder imports and the memory dart2wasm imports as
# `ffi.memory`, then drop the exports that only served the linking.
echo '(module (memory (export "memory") 1))' | wasm-tools parse -o "$WORK/memory.wasm"
wasm-tools parse "$SRC/embedder.wat" -o "$WORK/embedder.wasm"
FEATURES="--enable-gc --enable-reference-types --enable-multivalue
  --enable-exception-handling --enable-nontrapping-float-to-int
  --enable-sign-ext --enable-bulk-memory"
wasm-merge $FEATURES \
  "$WORK/memory.wasm" ffi "$WORK/embedder.wasm" dart "$WORK/dart.wasm" main \
  -o "$WORK/merged.wasm"

# Wasmtime implements exnref exceptions, not the legacy `try`/`catch` that
# dart2wasm emits.
wasm-opt $FEATURES --closed-world --traps-never-happen \
  --remove-exports='string*,i64ToString,stack*,print,$invokeMain' \
  --translate-to-exnref -O3 --strip-debug --strip-producers \
  "$WORK/merged.wasm" -o "$WORK/core.wasm"

if wasm-tools print "$WORK/core.wasm" | grep -q '(import "dart"'; then
  echo "error: unimplemented dart.* embedder import; extend embedder.wat" >&2
  wasm-tools print "$WORK/core.wasm" | grep '(import "dart"' >&2
  exit 1
fi

wasm-tools component embed "$SRC/../wit/bench.wit" --world deltablue \
  "$WORK/core.wasm" -o "$WORK/embedded.wasm"
wasm-tools component new "$WORK/embedded.wasm" \
  --adapt wasi_snapshot_preview1="$ADAPTER" -o "$OUT"
wasm-tools validate --features all "$OUT"
rm -rf "$WORK"
