FROM ubuntu:bionic
ARG REPOSITORY=https://github.com/WebAssembly/wasm-c-api
ARG REVISION=branch-heads/7.6

# The V8 build process untar's an archive; this causes issues inside some container frameworks, like
# `Cannot change ownership to uid 65002, gid 100005: Invalid argument` (see
# https://github.com/containers/buildah/issues/1702). Adding this flag to tar avoids the isue.
ENV TAR_OPTIONS=--no-same-owner

# Install dependencies for building v8; see
# https://github.com/WebAssembly/wasm-c-api/blob/master/Dockerfile.
RUN apt-get update && apt-get install -y \
    apt-utils \
    clang \
    cmake \
    curl \
    git \
    libc++-dev \
    libc++abi-dev \
    libglib2.0-dev \
    libgmp-dev \
    ninja-build \
    python

# Setup the wasm-c-api; TODO checkout a specific version of the wasm-c-api here.
WORKDIR /usr/src
RUN git clone ${REPOSITORY} wasm-c-api
WORKDIR /usr/src/wasm-c-api

# Build v8 as a static library; override the checked out V8 version using the passed $REVISION.
RUN make v8-checkout V8_VERSION=${REVISION}
RUN make v8

# Create the engine shared library and place it in the location `sightglass` expects.
COPY patch.diff .
RUN git apply patch.diff
COPY wasm-bench.cc ./src
RUN make out/libengine.so
RUN cp out/libengine.so /libengine.so
