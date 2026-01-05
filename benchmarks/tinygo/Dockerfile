ARG TINYGO_VERSION=0.40.0
FROM tinygo/tinygo:${TINYGO_VERSION}

WORKDIR /usr/src

# Copy all benchmark source and build script
COPY --chown=tinygo:tinygo json json
COPY --chown=tinygo:tinygo regex regex
COPY --chown=tinygo:tinygo build.sh .

# Create output directory as root and set permissions
USER root
RUN mkdir -p /benchmark && chown tinygo:tinygo /benchmark

# Switch back to tinygo user and build
USER tinygo
RUN OUTPUT_DIR=/benchmark ./build.sh

# Output goes to /benchmark where sightglass expects it
