FROM ubuntu:bionic

COPY . /

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
	apt-utils build-essential \
	curl ca-certificates \
    psmisc lsof \
    npm nano

ENV LD_LIBRARY_PATH=/usr/local/lib

RUN curl -sS -L -O https://static.rust-lang.org/dist/rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz \
	&& tar xzf rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz \
	&& cd rust-1.34.1-x86_64-unknown-linux-gnu \
	&& ./install.sh \
	&& cd .. \
	&& rm -rf rust-1.34.1-x86_64-unknown-linux-gnu rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz
ENV PATH=/usr/local/bin:$PATH

WORKDIR "/sg-view"
RUN pwd && ls -l && npm install

WORKDIR "/sg-history"
RUN pwd && ls -l && cargo build

WORKDIR "/"
