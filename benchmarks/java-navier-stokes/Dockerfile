# Java -> Wasm GC core module (TeaVM) -> WASI shim merged in (Binaryen) -> WASI
# component (wasm-tools + preview1 command adapter).
FROM eclipse-temurin:21.0.8_9-jdk-noble@sha256:03b731a231066bd04ada0a7ee9003cdbc1460083344f9277245b233e5ffcbf47

ARG TEAVM_VERSION=0.16.0
ARG BINARYEN_VERSION=132
ARG WASM_TOOLS_VERSION=1.247.0
ARG ADAPTER_WASMTIME_VERSION=37.0.0

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl ca-certificates; \
    rm -rf /var/lib/apt/lists/*

# TeaVM, without teavm-jso-impl: that plugin rewrites the module to import a JS
# runtime (strings, exceptions, interop).
WORKDIR /teavm
RUN set -eux; \
    for a in tooling core classlib interop platform jso jso-apis \
             metaprogramming-api extension-apis extension-spi extension-spi-util \
             relocated-libs-asm relocated-libs-asm-tree relocated-libs-asm-commons \
             relocated-libs-hppc; do \
      curl -sSfL -o "teavm-$a.jar" \
        "https://repo1.maven.org/maven2/org/teavm/teavm-$a/${TEAVM_VERSION}/teavm-$a-${TEAVM_VERSION}.jar"; \
    done

RUN set -eux; \
    case "$(uname -m)" in \
      x86_64) ARCH=x86_64 ;; \
      aarch64|arm64) ARCH=aarch64 ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    cd /tmp; \
    curl -sSfL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-${ARCH}-linux.tar.gz" \
      | tar -xz; \
    cp binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt binaryen-version_${BINARYEN_VERSION}/bin/wasm-merge /usr/local/bin/; \
    curl -sSfL "https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}/wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux.tar.gz" \
      | tar -xz; \
    cp wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux/wasm-tools /usr/local/bin/; \
    rm -rf /tmp/*; \
    curl -sSfL -o /teavm/wasi_snapshot_preview1.command.wasm \
      "https://github.com/bytecodealliance/wasmtime/releases/download/v${ADAPTER_WASMTIME_VERSION}/wasi_snapshot_preview1.command.wasm"; \
    wasm-opt --version; \
    wasm-tools --version

WORKDIR /work
COPY tools ./tools
COPY src ./src
COPY shim.wat ./
COPY wit ./wit

# Binaryen features: Wasm GC plus what TeaVM emits. Everything else stays
# disabled so the optimizer does not introduce newer proposals (e.g. exact
# references) that the engine does not enable.
ENV WASM_FEATURES="--mvp-features --enable-gc --enable-reference-types --enable-exception-handling --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext --enable-multivalue --enable-mutable-globals"

RUN set -eux; \
    CP=$(ls /teavm/*.jar | tr '\n' ':'); \
    javac --release 21 -cp "$CP" -d classes tools/TeaVMBuild.java src/sightglass/*.java; \
    java -cp "classes:$CP" TeaVMBuild sightglass.Main out teavm.wasm; \
    \
    # Route TeaVM's JS-loader imports (memory, heap bounds, Math.sqrt, clock)
    # to `shim.wat`, and drop TeaVM's JS-facing exports.
    wasm-tools print out/teavm.wasm \
      | sed -E 's/\(import "(env|teavmMemory|teavmMath|teavmDate)" /(import "shim" /; /^  \(export "teavm/d' \
      | wasm-tools parse -o renamed.wasm; \
    wasm-tools parse shim.wat -o shim.wasm; \
    wasm-merge $WASM_FEATURES shim.wasm shim renamed.wasm main -o merged.wasm; \
    wasm-tools print merged.wasm \
      | sed -E '/^  \(export "(heapOffset|maxSize|notifyHeapResized|sqrt|currentTimeMillis)"/d' \
      | wasm-tools parse -o merged-exports.wasm; \
    wasm-opt $WASM_FEATURES --closed-world -O3 --strip-debug --strip-producers \
      merged-exports.wasm -o core.wasm; \
    \
    wasm-tools component embed wit/bench.wit --world navier-stokes core.wasm -o embedded.wasm; \
    wasm-tools component new embedded.wasm \
      --adapt wasi_snapshot_preview1=/teavm/wasi_snapshot_preview1.command.wasm \
      -o java-navier-stokes.wasm; \
    wasm-tools validate --features all java-navier-stokes.wasm; \
    mkdir -p /benchmark; \
    cp java-navier-stokes.wasm /benchmark/
