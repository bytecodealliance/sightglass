FROM ubuntu:bionic

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
	apt-utils build-essential gpg-agent \
	curl ca-certificates wget software-properties-common \
    psmisc lsof git cmake nano

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key|apt-key add -

RUN add-apt-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-8 main"

RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
	clang-8 lldb-8 lld-8

RUN curl -sL https://github.com/CraneStation/wasi-sdk/releases/download/wasi-sdk-6/libclang_rt.builtins-wasm32-wasi-6.0.tar.gz  | tar x -zf - -C /usr/lib/llvm-8/lib/clang/8.0.1

RUN ln -s /usr/bin/wasm-ld-8 /usr/bin/wasm-ld

ENV LD_LIBRARY_PATH=/usr/local/lib

RUN curl -sS -L -O https://static.rust-lang.org/dist/rust-1.40.0-x86_64-unknown-linux-gnu.tar.gz \
	&& tar xzf rust-1.40.0-x86_64-unknown-linux-gnu.tar.gz \
	&& cd rust-1.40.0-x86_64-unknown-linux-gnu \
	&& ./install.sh \
	&& cd .. \
	&& rm -rf rust-1.40.0-x86_64-unknown-linux-gnu rust-1.40.0-x86_64-unknown-linux-gnu.tar.gz

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash

RUN curl -sL https://deb.nodesource.com/setup_12.x | bash

RUN apt-get install --yes nodejs
RUN npm install wasi

ENV PATH=/usr/local/bin:$PATH

CMD ["/bin/bash"]

WORKDIR "/sightglass_runner/"
