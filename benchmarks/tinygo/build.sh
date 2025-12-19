#!/bin/sh
set -e

# Configuration
TINYGO="${TINYGO:-tinygo}"
OUTPUT_DIR="${OUTPUT_DIR:-$(pwd)}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

cd "$SCRIPT_DIR"

# Auto-discover benchmarks: any directory with a main.go
for dir in */; do
    # Remove trailing slash
    benchmark=$(basename "$dir")

    # Skip if not a benchmark directory (no main.go)
    [ ! -f "$dir/main.go" ] && continue

    echo "Building TinyGo $benchmark benchmark..."

    cd "$dir"

    # Build with TinyGo
    "$TINYGO" build -o "$OUTPUT_DIR/tinygo-$benchmark.wasm" -target=wasi -opt=2 -gc=leaking .

    cd "$SCRIPT_DIR"

    echo "> Built tinygo-$benchmark.wasm"
done

echo "All TinyGo benchmarks built successfully"
