# Reproducible build of the hoot-earley-boyer benchmark.
#
# Stage 1 builds Guile and Hoot from release tarballs and compiles the Scheme
# sources to a WasmGC core module. Stage 2 links in host.wat (Hoot's JS runtime
# imports, reimplemented over WASI preview1) and makes a WASI command component.

FROM debian:bookworm-20251229-slim@sha256:d5d3f9c23164ea16f31852f95bd5959aad1c5e854332fe00f7b3a20fcc9f635c AS hoot

ARG GUILE_VERSION=3.0.11
ARG HOOT_VERSION=0.9.0

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      ca-certificates curl xz-utils make gcc libc6-dev pkg-config \
      libgmp-dev libunistring-dev libffi-dev libgc-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /build
RUN curl -sSLO https://ftp.gnu.org/gnu/guile/guile-${GUILE_VERSION}.tar.xz \
 && tar xf guile-${GUILE_VERSION}.tar.xz \
 && cd guile-${GUILE_VERSION} \
 && ./configure --prefix=/usr/local --disable-static \
 && make -j"$(nproc)" && make install && ldconfig \
 && cd .. && rm -rf guile-${GUILE_VERSION}*
RUN curl -sSLO https://files.spritely.institute/releases/guile-hoot/guile-hoot-${HOOT_VERSION}.tar.gz \
 && tar xzf guile-hoot-${HOOT_VERSION}.tar.gz \
 && cd guile-hoot-${HOOT_VERSION} \
 && ./configure --prefix=/usr/local \
 && make -j"$(nproc)" GUILE_AUTO_COMPILE=0 && make install \
 && cd .. && rm -rf guile-hoot-${HOOT_VERSION}*

WORKDIR /src
COPY main.scm earley.scm nboyer.scm ./
# `standalone` mode exports a `main` command entry point. `-g0` drops debug
# metadata.
RUN GUILE_AUTO_COMPILE=0 guild compile-wasm --mode=standalone -g0 \
      -o hoot.wasm main.scm

FROM debian:bookworm-20251229-slim@sha256:d5d3f9c23164ea16f31852f95bd5959aad1c5e854332fe00f7b3a20fcc9f635c AS link

ARG WASM_TOOLS_VERSION=1.247.0
ARG BINARYEN_VERSION=124
ARG ADAPTER_WASMTIME_VERSION=37.0.0

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl ca-certificates gawk; \
    rm -rf /var/lib/apt/lists/*; \
    case "$(uname -m)" in \
      x86_64) ARCH=x86_64 ;; \
      aarch64|arm64) ARCH=aarch64 ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    curl -sSL "https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}/wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux.tar.gz" \
      | tar xz; \
    cp "wasm-tools-${WASM_TOOLS_VERSION}-${ARCH}-linux/wasm-tools" /usr/local/bin/; \
    curl -sSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-${ARCH}-linux.tar.gz" \
      | tar xz -C /opt; \
    ln -s /opt/binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt /usr/local/bin/; \
    ln -s /opt/binaryen-version_${BINARYEN_VERSION}/bin/wasm-merge /usr/local/bin/; \
    curl -sSL -o /adapter.wasm \
      "https://github.com/bytecodealliance/wasmtime/releases/download/v${ADAPTER_WASMTIME_VERSION}/wasi_snapshot_preview1.command.wasm"; \
    wasm-tools --version; wasm-opt --version

WORKDIR /work
COPY link.sh stubgen.awk host.wat ./
COPY wit ./wit
COPY --from=hoot /src/hoot.wasm ./
RUN mkdir -p /benchmark \
 && ./link.sh hoot.wasm /benchmark/hoot-earley-boyer.wasm /adapter.wasm
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
