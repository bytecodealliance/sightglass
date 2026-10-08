# Reproducible build of the dotnet-raytracer benchmark: C# -> NativeAOT-LLVM ->
# WASIp2 component, driven by componentize-dotnet.
FROM ubuntu:24.04@sha256:534baea6a22c03a63003dbc8dbe78fe34bc0d7e595d9a9dc9834884ff530eb55

ARG DOTNET_SDK_VERSION=10.0.401
ARG DOTNET_SDK_SHA512_X64=51c8b999af9e8dd9998c9edc5944e19a90788862068acd38694e098889054ce8c23d4f0c5cccfa16bf187d044562359e5ee69a9f8ad0bbe913ba90311fbce25b
ARG DOTNET_SDK_SHA512_ARM64=58ace73ced6b4360754689a686bdfb8a317f4da6cb8bb416dbc7d0ba9f47e43e3c09f5eb1f1a1cfaacbd10df9558da4882bf2a5e195d6ab56a02c1f9f76102ed
# The version NativeAOT-LLVM 10.0.0-rc.1.26306.1 expects.
ARG WASI_SDK_VERSION=29.0
ARG WASI_SDK_SHA256_X64=87d1d1a2879d139cdc624b968efad3d4a97b8078cdff95e63ac88ecafd1a0171
ARG WASI_SDK_SHA256_ARM64=052ad773397dc9e5aa99fb4cfef694175e6b1e81bb2ad1d3c8e7b3fc81441b7c

ENV DOTNET_ROOT=/opt/dotnet \
    PATH=/opt/dotnet:$PATH \
    DOTNET_CLI_TELEMETRY_OPTOUT=1 \
    DOTNET_NOLOGO=1 \
    DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends ca-certificates curl libstdc++6 zlib1g; \
    rm -rf /var/lib/apt/lists/*; \
    case "$(uname -m)" in \
      x86_64) SDK_ARCH=x64; SDK_SHA=$DOTNET_SDK_SHA512_X64; WASI_ARCH=x86_64; WASI_SHA=$WASI_SDK_SHA256_X64 ;; \
      aarch64) SDK_ARCH=arm64; SDK_SHA=$DOTNET_SDK_SHA512_ARM64; WASI_ARCH=arm64; WASI_SHA=$WASI_SDK_SHA256_ARM64 ;; \
      *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;; \
    esac; \
    curl -sSfL -o /tmp/dotnet.tar.gz \
      "https://builds.dotnet.microsoft.com/dotnet/Sdk/${DOTNET_SDK_VERSION}/dotnet-sdk-${DOTNET_SDK_VERSION}-linux-${SDK_ARCH}.tar.gz"; \
    echo "${SDK_SHA}  /tmp/dotnet.tar.gz" | sha512sum -c -; \
    mkdir -p /opt/dotnet; \
    tar -xzf /tmp/dotnet.tar.gz -C /opt/dotnet; \
    rm /tmp/dotnet.tar.gz; \
    dotnet --version; \
    # componentize-dotnet would download the x86_64 wasi-sdk here regardless of
    # host architecture; pre-installing the right one makes it skip that.
    curl -sSfL -o /tmp/wasi-sdk.tar.gz \
      "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION%%.*}/wasi-sdk-${WASI_SDK_VERSION}-${WASI_ARCH}-linux.tar.gz"; \
    echo "${WASI_SHA}  /tmp/wasi-sdk.tar.gz" | sha256sum -c -; \
    mkdir -p "/root/.wasi-sdk/wasi-sdk-${WASI_SDK_VERSION}"; \
    tar -xzf /tmp/wasi-sdk.tar.gz -C "/root/.wasi-sdk/wasi-sdk-${WASI_SDK_VERSION}" --strip-components=1; \
    rm /tmp/wasi-sdk.tar.gz

WORKDIR /src

# Restore in its own layer so that editing the C# does not refetch packages.
COPY nuget.config dotnet-raytracer.csproj ./
COPY wit ./wit
RUN dotnet restore dotnet-raytracer.csproj

COPY src ./src
# componentize-dotnet hooks `Publish` onto `Build`; this produces the final
# component (wasm-component-ld adds the `bench` import and `wasi:cli/run`).
RUN set -eux; \
    dotnet build dotnet-raytracer.csproj -c Release --no-restore; \
    mkdir -p /benchmark; \
    cp bin/Release/net10.0/wasi-wasm/native/dotnet-raytracer.wasm /benchmark/
# We output the Wasm file to the `/benchmark` directory, where the client
# expects it.
