FROM ubuntu:bionic
ENV RUST_VERISON 1.44.1

RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
	apt-utils build-essential gpg-agent \
	curl ca-certificates wget software-properties-common \
	psmisc lsof git cmake nano

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key|apt-key add -

RUN add-apt-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-10 main"

RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
	clang-10 lldb-10 lld-10

RUN curl -k -O -L https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-11/wasi-sdk_11.0_amd64.deb
RUN apt install ./wasi-sdk_11.0_amd64.deb

ENV LD_LIBRARY_PATH=/usr/local/lib

# https://forge.rust-lang.org/infra/other-installation-methods.html
RUN curl -sS -L -O https://static.rust-lang.org/dist/rust-${RUST_VERISON}-x86_64-unknown-linux-gnu.tar.gz \
	&& tar xzf rust-${RUST_VERISON}-x86_64-unknown-linux-gnu.tar.gz \
	&& cd rust-${RUST_VERISON}-x86_64-unknown-linux-gnu \
	&& ./install.sh \
	&& cd .. \
	&& rm -rf rust-${RUST_VERISON}-x86_64-unknown-linux-gnu rust-${RUST_VERISON}-x86_64-unknown-linux-gnu.tar.gz

RUN curl -sL https://deb.nodesource.com/setup_14.x | bash

RUN apt-get install --yes nodejs
RUN npm install wasi

ENV PATH=/usr/local/bin:$PATH

CMD ["/bin/bash"]

WORKDIR "/sightglass_runner/"
