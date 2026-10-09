# Rust 1.88.0 is the toolchain StarlingMonkey's `rust-toolchain.toml` asks for.
FROM rust:1.88.0-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0

# StarlingMonkey v0.3.0. Its CMake build downloads further tools at versions pinned
# by this commit: wasi-sdk 30, wasmtime 42.0.1 (for `wasmtime wizer`), wasm-tools
# 1.235.0, binaryen 123, weval 0.4.1, corrosion 0.5.1, CPM 0.42.0, OpenSSL 3.0.17.
ARG STARLINGMONKEY_COMMIT=9dda8ba7fcda2e17c6795d402f0478cf4c1f7f37
# The prebuilt SpiderMonkey that v0.3.0 uses (tag FIREFOX_147_0_4_RELEASE_STARLING).
ARG SPIDERMONKEY_SHA256=c57dc83d93dc04198882b44fea49cd3cb01e0b267bf99fae3b058cb158da684b
ARG CMAKE_VERSION=3.31.8
ARG WIT_BINDGEN_VERSION=0.52.0
ARG CBINDGEN_VERSION=0.29.0

ENV CARGO_BUILD_JOBS=6
RUN rustup target add wasm32-wasip1

# StarlingMonkey's build expects `cbindgen` on PATH and otherwise installs an
# unpinned one.
RUN cargo install --locked cbindgen --version ${CBINDGEN_VERSION}

WORKDIR /opt
RUN set -eux; \
    case "$(uname -m)" in \
      x86_64) ARCH=x86_64 ;; \
      aarch64) ARCH=aarch64 ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    curl -sSfL "https://github.com/Kitware/CMake/releases/download/v${CMAKE_VERSION}/cmake-${CMAKE_VERSION}-linux-${ARCH}.tar.gz" | tar xz; \
    ln -s /opt/cmake-${CMAKE_VERSION}-linux-${ARCH}/bin/cmake /usr/local/bin/cmake; \
    curl -sSfL "https://github.com/bytecodealliance/wit-bindgen/releases/download/v${WIT_BINDGEN_VERSION}/wit-bindgen-${WIT_BINDGEN_VERSION}-${ARCH}-linux.tar.gz" | tar xz; \
    ln -s /opt/wit-bindgen-${WIT_BINDGEN_VERSION}-${ARCH}-linux/wit-bindgen /usr/local/bin/wit-bindgen; \
    curl -sSfL -o spidermonkey.tar.gz \
      "https://github.com/bytecodealliance/starlingmonkey/releases/download/libspidermonkey_FIREFOX_147_0_4_RELEASE_STARLING/spidermonkey-static-release.tar.gz"; \
    echo "${SPIDERMONKEY_SHA256}  spidermonkey.tar.gz" | sha256sum -c; \
    tar xzf spidermonkey.tar.gz; \
    rm spidermonkey.tar.gz
ENV SPIDERMONKEY_BINARIES=/opt/spidermonkey-dist-release

WORKDIR /usr/src
RUN git init starlingmonkey \
 && git -C starlingmonkey fetch --depth 1 https://github.com/bytecodealliance/StarlingMonkey ${STARLINGMONKEY_COMMIT} \
 && git -C starlingmonkey checkout FETCH_HEAD

# The WASI 0.2.3 WIT that StarlingMonkey vendors, plus the `bench` import.
COPY host-api /usr/src/benchmark/host-api
RUN cp -r starlingmonkey/host-apis/wasi-0.2.3/wit/deps benchmark/host-api/wit/ \
 && wit-bindgen c benchmark/host-api/wit --world benchmark \
      --rename wasi:cli/run@0.2.3=benchmark_run \
      --out-dir benchmark/host-api/bindings

# `starling-raw.wasm` with the sightglass host API and `bench.cpp` builtin. The
# fetch builtins and the debugger need `wasi:http` and sockets, which this host
# API omits. `WPT_ROOT` only stops configure from cloning the WPT test suite.
COPY CMakeLists.txt bench.cpp /usr/src/benchmark/
RUN STARLINGMONKEY_DIR=/usr/src/starlingmonkey \
    HOST_API=/usr/src/benchmark/host-api \
    WPT_ROOT=/nonexistent \
    cmake -S benchmark -B build -DCMAKE_BUILD_TYPE=Release \
      -DENABLE_JS_DEBUGGER=OFF \
      -DENABLE_BUILTIN_WEB_FETCH=OFF \
      -DENABLE_BUILTIN_WEB_FETCH_FETCH_EVENT=OFF \
 && cmake --build build --target starling-raw.wasm --parallel 6

COPY build.sh /usr/src/benchmark/
COPY js /usr/src/benchmark/js
COPY warmup /usr/src/benchmark/warmup
RUN /usr/src/benchmark/build.sh
# We output the Wasm files to the `/benchmark` directory, where the client expects it.
