#!/usr/bin/env bash
# Usage: link.sh <hoot.wasm> <out.wasm> <adapter.wasm>
#
# Turns a Hoot standalone module into a WASI command component. Hoot imports
# its runtime from JavaScript (namespaces "rt", "io", "debug"); host.wat
# implements the ones this program uses on top of WASI preview1, and every
# other one becomes a trapping stub.
set -euo pipefail
in=$1 out=$2 adapter=$3
here=$(cd "$(dirname "$0")" && pwd)
t=$(mktemp -d)
trap 'rm -rf "$t"' EXIT

# Binaryen would otherwise emit exact reference types, which Wasmtime rejects.
feat="--all-features --disable-custom-descriptors"

wasm-tools print "$in" \
  | sed -E 's/\(import "(rt|io|debug)" /(import "host" /' \
  | wasm-tools parse -o "$t/hoot.wasm"
wasm-tools parse "$here/host.wat" -o "$t/host.wasm"
wasm-merge $feat "$t/hoot.wasm" hoot "$t/host.wasm" host -o "$t/m1.wasm"

{ echo "(module"; wasm-tools print "$t/m1.wasm" | awk -f "$here/stubgen.awk"; echo ")"; } \
  > "$t/stubs.wat"
wasm-tools parse "$t/stubs.wat" -o "$t/stubs.wasm"
wasm-merge $feat "$t/m1.wasm" main "$t/stubs.wasm" host -o "$t/m2.wasm"

# Keep only `_start` (Hoot's `main` command entry) and the shim's memory, which
# the preview1 adapter needs as the module's exported "memory".
main=$(wasm-tools print "$t/m2.wasm" | sed -nE 's/^ *\(export "main" \(func ([0-9]+)\)\)/\1/p')
mem=$(wasm-tools print "$t/m2.wasm" | sed -nE 's/^ *\(export "memory" \(memory ([0-9]+)\)\)/\1/p')
wasm-tools print "$t/m2.wasm" \
  | grep -v -E '^ *\(export "' \
  | awk -v m="$main" -v mem="$mem" '/^  \(start /{
      print "  (export \"_start\" (func " m "))"
      print "  (export \"memory\" (memory " mem "))"
    } {print}' \
  | wasm-tools parse -o "$t/core.wasm"

wasm-opt $feat -O2 "$t/core.wasm" -o "$t/core-opt.wasm"
wasm-tools component embed "$here/wit/bench.wit" --world hoot "$t/core-opt.wasm" -o "$t/embedded.wasm"
wasm-tools component new "$t/embedded.wasm" \
  --adapt wasi_snapshot_preview1="$adapter" -o "$out"
wasm-tools validate --features all "$out"
