# Reproducible build of the ocaml-raytrace benchmark.
#
# Stage 1 compiles OCaml to a WasmGC core module with wasm_of_ocaml's WASI
# target. Stage 2 turns that into a WASI component with wasm-tools and the WASI
# preview1 command adapter.

# Stage 1: OCaml -> WasmGC core module.
# ocaml/opam:debian-12-ocaml-5.4 (OCaml 5.4.1, opam 2.1.6).
FROM ocaml/opam:debian-12-ocaml-5.4@sha256:ba37cf7a29709fa2f19124fda8f4cabdea3156a648276fc98c9d528998ae2e59 AS ocaml-build

ARG BINARYEN_VERSION=133
ARG OPAM_REPOSITORY_COMMIT=ac27950e5eac6c981ad809dff370c937820b7893

USER root
RUN set -eux; \
    case "$(uname -m)" in \
      x86_64) B_ARCH=x86_64-linux ;; \
      aarch64|arm64) B_ARCH=aarch64-linux ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    curl -sSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-${B_ARCH}.tar.gz" \
      | tar -xz -C /opt; \
    ln -s /opt/binaryen-version_${BINARYEN_VERSION}/bin/* /usr/local/bin/; \
    wasm-opt --version; \
    mkdir /src; chown opam /src
USER opam

# `--assume-depexts` keeps opam from apt-installing Debian's older binaryen for
# `conf-binaryen`; the release above provides it.
RUN set -eux; \
    cd ~/opam-repository; \
    git fetch -q origin "$OPAM_REPOSITORY_COMMIT"; \
    git checkout -q "$OPAM_REPOSITORY_COMMIT"; \
    opam update -q default; \
    opam install -y --assume-depexts \
      dune.3.24.2 js_of_ocaml-compiler.6.4.1 wasm_of_ocaml-compiler.6.4.1

WORKDIR /src
COPY --chown=opam raytrace.ml bench.wat ./
# `-no-check-prims` because `bench_start`/`bench_end` are only provided by
# bench.wat at wasm_of_ocaml link time.
RUN set -eux; \
    eval $(opam env); \
    ocamlc -no-check-prims -o raytrace.byte raytrace.ml; \
    wasm_of_ocaml compile --enable wasi --opt=3 bench.wat raytrace.byte -o raytrace.js; \
    cp raytrace.assets/code.wasm core.wasm

# Stage 2: core module -> WASI component.
FROM debian:bookworm-slim@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587 AS component-build

# The adapter targets WASI 0.2.6 (matches sightglass).
ARG WASM_TOOLS_VERSION=1.247.0
ARG ADAPTER_WASMTIME_VERSION=37.0.0
WORKDIR /work

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl ca-certificates; \
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

COPY --from=ocaml-build /src/core.wasm core.wasm
COPY wit ./wit

# wasm_of_ocaml's WASI output already exports `_start`, which the command
# adapter binds to `wasi:cli/run`.
RUN set -eux; \
    wasm-tools component embed wit/bench.wit --world raytrace core.wasm -o embedded.wasm; \
    wasm-tools component new embedded.wasm \
      --adapt wasi_snapshot_preview1=wasi_snapshot_preview1.command.wasm \
      -o ocaml-raytrace.wasm; \
    wasm-tools validate --features all ocaml-raytrace.wasm; \
    mkdir -p /benchmark; \
    cp ocaml-raytrace.wasm /benchmark/
