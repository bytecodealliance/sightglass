FROM rust:1.60
ARG REPOSITORY=https://github.com/bytecodealliance/wasmtime/
ARG REVISION=main
ARG BUILD="cargo build -p wasmtime-bench-api"
ARG FLAGS="--release"

WORKDIR /usr/src
# Clone a single commit (actually, a "revision"--tags and branches work as well).
RUN git init
RUN git remote add origin ${REPOSITORY}
RUN git fetch --depth 1 origin ${REVISION}
RUN git checkout FETCH_HEAD
RUN git submodule update --init --depth 1
# Build the engine library.
RUN ${BUILD} ${FLAGS}
RUN mv target/release/libwasmtime_bench_api.so /libengine.so
