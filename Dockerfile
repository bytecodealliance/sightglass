FROM ubuntu:bionic

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
	apt-utils build-essential gpg-agent \
	curl ca-certificates wget software-properties-common \
    psmisc lsof git cmake \
    npm nano

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key|apt-key add -

RUN add-apt-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-8 main"
#RUN add-apt-repository "deb-src http://apt.llvm.org/bionic/ llvm-toolchain-bionic-8 main"

RUN apt-get update \ 
	&& apt-get install -y --no-install-recommends \ 
	clang-8 lldb-8 lld-8

RUN curl -sL https://github.com/CraneStation/wasi-sdk/releases/download/wasi-sdk-6/libclang_rt.builtins-wasm32-wasi-6.0.tar.gz  | tar x -zf - -C /usr/lib/llvm-8/lib/clang/8.0.1

RUN ln -s /usr/bin/wasm-ld-8 /usr/bin/wasm-ld

#	libllvm-8-ocaml-dev libllvm8 llvm-8 llvm-8-dev llvm-8-runtime \
#	clang-8 clang-tools-8 libclang-common-8-dev libclang-8-dev libclang1-8 clang-format-8 python-clang-8 \
# 	lldb-8

# Setting a consistent LD_LIBRARY_PATH across the entire environment prevents unnecessary cargo
# rebuilds.
ENV LD_LIBRARY_PATH=/usr/local/lib

RUN curl -sS -L -O https://static.rust-lang.org/dist/rust-1.37.0-x86_64-unknown-linux-gnu.tar.gz \
	&& tar xzf rust-1.37.0-x86_64-unknown-linux-gnu.tar.gz \
	&& cd rust-1.37.0-x86_64-unknown-linux-gnu \
	&& ./install.sh \
	&& cd .. \
	&& rm -rf rust-1.37.0-x86_64-unknown-linux-gnu rust-1.37.0-x86_64-unknown-linux-gnu.tar.gz

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash

ENV PATH=/usr/local/bin:$PATH

CMD ["/bin/bash"]

WORKDIR "/sightglass_runner/"

