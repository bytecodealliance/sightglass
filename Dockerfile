FROM ubuntu:bionic

COPY webui /webui/

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
	apt-utils build-essential \
	curl ca-certificates \
    psmisc lsof \
    npm nano

# Setting a consistent LD_LIBRARY_PATH across the entire environment prevents unnecessary cargo
# rebuilds.
ENV LD_LIBRARY_PATH=/usr/local/lib

#RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN curl -sS -L -O https://static.rust-lang.org/dist/rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz \
	&& tar xzf rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz \
	&& cd rust-1.34.1-x86_64-unknown-linux-gnu \
	&& ./install.sh \
	&& cd .. \
	&& rm -rf rust-1.34.1-x86_64-unknown-linux-gnu rust-1.34.1-x86_64-unknown-linux-gnu.tar.gz
ENV PATH=/usr/local/bin:$PATH


#ARG sg_view_port
#ARG sg_history_port

#RUN echo "Expose Ports: ${sg_view_port} ${sg_history_port}"
#EXPOSE ${sg_view_port}
#EXPOSE ${sg_history_port}

WORKDIR "/webui/sg-view"
RUN pwd && ls -l && npm install

WORKDIR "/webui/sg-history"
RUN pwd && ls -l && cargo build

WORKDIR "/"