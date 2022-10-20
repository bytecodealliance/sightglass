FROM ubuntu:20.04

RUN apt-get update
RUN apt-get -y install git python3 python3-pip python3-distutils curl sudo
RUN curl https://sh.rustup.rs | bash -s - -y
ENV PATH=/root/.cargo/bin:$PATH

# Build SpiderMonkey itself.
WORKDIR /usr/src
RUN git clone https://github.com/tschneidereit/spidermonkey-wasi-embedding
WORKDIR /usr/src/spidermonkey-wasi-embedding
ENV DEBIAN_FRONTEND=noninteractive
RUN ./build-engine.sh

RUN apt-get -y install wget
WORKDIR /opt
RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-14/wasi-sdk-14.0-linux.tar.gz
RUN tar zxvf wasi-sdk-14.0-linux.tar.gz
RUN ln -s wasi-sdk-14.0 wasi-sdk

RUN apt-get -y install xxd

WORKDIR /usr/src

RUN mkdir benchmark && cd benchmark/

COPY runtime.cpp .
COPY sightglass.h .
COPY build.sh .

RUN mkdir js
COPY js/marked.min.js js/marked.min.js
COPY js/main.js js/main.js

RUN ./build.sh
