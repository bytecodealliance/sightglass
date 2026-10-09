# Reproducible build of the dart-deltablue benchmark: dart2wasm (standalone
# mode) -> link Wasm embedder imports -> WASI component. See compile.sh.
FROM dart:3.13.5-sdk@sha256:8604cbd137b6d67c24f49b916f2aa9b67d71b539576e0178bf161d831c9e344f

ARG BINARYEN_VERSION=130
ARG WASM_TOOLS_VERSION=1.247.0
# The adapter targets WASI 0.2.6 (matches sightglass).
ARG ADAPTER_WASMTIME_VERSION=37.0.0

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl ca-certificates; \
    rm -rf /var/lib/apt/lists/*; \
    case "$(uname -m)" in \
      x86_64) ARCH=x86_64 ;; \
      aarch64|arm64) ARCH=aarch64 ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    mkdir -p /opt/tools; cd /opt/tools; \
    curl -sSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-${ARCH}-linux.tar.gz" \
      | tar -xz; \
    curl -sSL "https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}/wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux.tar.gz" \
      | tar -xz; \
    cp "wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux/wasm-tools" /usr/local/bin/; \
    curl -sSL -o wasi_snapshot_preview1.command.wasm \
      "https://github.com/bytecodealliance/wasmtime/releases/download/v${ADAPTER_WASMTIME_VERSION}/wasi_snapshot_preview1.command.wasm"

ENV PATH="/opt/tools/binaryen-version_${BINARYEN_VERSION}/bin:${PATH}"
ENV ADAPTER=/opt/tools/wasi_snapshot_preview1.command.wasm

WORKDIR /src
COPY compile.sh ./
COPY src ./src
COPY wit ./wit
RUN mkdir -p /benchmark && ./compile.sh /benchmark/dart-deltablue.wasm
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
