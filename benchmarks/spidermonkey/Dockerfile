FROM ubuntu:24.04

# WASI SDK version to use - update this to upgrade
ARG WASI_SDK_VERSION=29

RUN apt-get update
RUN apt-get -y install git curl wget xxd
ENV PATH=/root/.cargo/bin:$PATH

# Build SpiderMonkey itself.
WORKDIR /usr/src
RUN git clone -b rev_b02d76023a15a3fa8c8f54bff5dac91099669003 \
    --single-branch https://github.com/bytecodealliance/spidermonkey-wasi-embedding
WORKDIR /usr/src/spidermonkey-wasi-embedding
ENV DEBIAN_FRONTEND=noninteractive
RUN ./download-engine.sh

WORKDIR /opt
# Download the appropriate wasi-sdk for the target architecture
RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
    WASI_ARCH="x86_64"; \
    elif [ "$ARCH" = "aarch64" ]; then \
    WASI_ARCH="arm64"; \
    else \
    echo "Unsupported architecture: $ARCH" && exit 1; \
    fi && \
    wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION}/wasi-sdk-${WASI_SDK_VERSION}.0-${WASI_ARCH}-linux.tar.gz && \
    tar zxvf wasi-sdk-${WASI_SDK_VERSION}.0-${WASI_ARCH}-linux.tar.gz && \
    ln -s wasi-sdk-${WASI_SDK_VERSION}.0-${WASI_ARCH}-linux wasi-sdk

WORKDIR /usr/src

# Copy runtime, sightglass header, JS files, and build script
COPY runtime.cpp .
COPY sightglass.h .
COPY js js
COPY build.sh .
RUN chmod +x build.sh

RUN ./build.sh
# We output the Wasm files to the `/benchmark` directory, where the client expects it.
