# Reproducible build of the kotlin-richards benchmark.
#
# Stage 1 compiles the Kotlin sources to an optimized WasmGC core module via the
# Kotlin/Wasm `wasmWasi` target. Stage 2 turns that core module into a WASI
# *component* with wasm-tools and the WASI preview1 command adapter.

# ---------------------------------------------------------------------------
# Stage 1: Kotlin -> optimized WasmGC core module.
# ---------------------------------------------------------------------------
FROM gradle:8.13-jdk21 AS kotlin-build
WORKDIR /src

# Warm the Kotlin/Native + Binaryen toolchain and dependency caches in a layer
# that does not depend on our sources, so editing the Kotlin only re-runs the
# (fast) compile below.
COPY settings.gradle.kts gradle.properties build.gradle.kts ./
RUN mkdir -p src/wasmWasiMain/kotlin \
 && printf 'fun main() {}\n' > src/wasmWasiMain/kotlin/Stub.kt \
 && gradle --no-daemon compileProductionExecutableKotlinWasmWasiOptimize \
 && rm -rf src

# Compile the real sources. Produces, with Binaryen already applied:
#   build/compileSync/wasmWasi/main/productionExecutable/optimized/kotlin-richards.wasm
COPY src ./src
RUN gradle --no-daemon compileProductionExecutableKotlinWasmWasiOptimize

# ---------------------------------------------------------------------------
# Stage 2: core module -> WASI component.
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim AS component-build

# Pinned tool versions. The adapter targets WASI 0.2.6 (matches sightglass).
ARG WASM_TOOLS_VERSION=1.247.0
ARG ADAPTER_WASMTIME_VERSION=37.0.0
WORKDIR /work

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl xz-utils ca-certificates; \
    rm -rf /var/lib/apt/lists/*; \
    case "$(uname -m)" in \
      x86_64) WT_ARCH=x86_64-linux ;; \
      aarch64|arm64) WT_ARCH=aarch64-linux ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    curl -sSL -o wasm-tools.tar.gz \
      "https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}/wasm-tools-${WASM_TOOLS_VERSION}-${WT_ARCH}.tar.gz"; \
    tar -xzf wasm-tools.tar.gz; \
    cp "wasm-tools-${WASM_TOOLS_VERSION}-${WT_ARCH}/wasm-tools" /usr/local/bin/; \
    wasm-tools --version; \
    curl -sSL -o wasi_snapshot_preview1.command.wasm \
      "https://github.com/bytecodealliance/wasmtime/releases/download/v${ADAPTER_WASMTIME_VERSION}/wasi_snapshot_preview1.command.wasm"

COPY --from=kotlin-build \
     /src/build/compileSync/wasmWasi/main/productionExecutable/optimized/kotlin-richards.wasm \
     core.wasm
COPY wit ./wit

# The Kotlin WASI executable exports `_initialize` (a WASI "reactor" entry that
# runs `fun main()`). Rename that export to `_start` so the preview1 *command*
# adapter binds it to `wasi:cli/run`'s `run`, which is how sightglass invokes a
# component benchmark. Then embed the world describing the `bench` import and
# create the component (the adapter supplies the WASI imports + `run` export).
RUN set -eux; \
    wasm-tools print core.wasm \
      | sed -E 's/\(export "_initialize" \(func /(export "_start" (func /' \
      | wasm-tools parse -o command.wasm; \
    wasm-tools component embed wit/bench.wit --world richards command.wasm -o embedded.wasm; \
    wasm-tools component new embedded.wasm \
      --adapt wasi_snapshot_preview1=wasi_snapshot_preview1.command.wasm \
      -o kotlin-richards.wasm; \
    wasm-tools validate --features all kotlin-richards.wasm; \
    mkdir -p /benchmark; \
    cp kotlin-richards.wasm /benchmark/
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
