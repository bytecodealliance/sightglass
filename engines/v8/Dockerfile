FROM ubuntu:latest
ARG REPOSITORY=https://github.com/WebAssembly/wasm-c-api
ARG REVISION=branch-heads/9.9

# Install necessary dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -yqq \
    apt-utils \
    curl \
    git \
    g++ \
    make \
    ninja-build \
    pkg-config \
    python3

# Setup V8 and build the `libwee8.a` library. TODO map REVISION to V8_VERSION.
WORKDIR /usr/src
COPY args.gn Makefile ./
RUN make v8-checkout
RUN make wee8

# Build the Sightglass engine.
COPY src ./src
RUN make build/libengine.so
RUN cp build/libengine.so /libengine.so
