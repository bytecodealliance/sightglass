#!/usr/bin/env bash

# Clean any artifacts remaining after using `build.sh` or `build-all.sh`. This attempts to remove
# the following kinds of `sightglass-benchmark` artifacts:
# - any temporary files
# - any Docker containers
# - any Docker images
#
# Usage: ./clean.sh

set -e
(set -x; rm -vf /tmp/sightglass-benchmark-*)
CONTAINERS=$(docker ps -a | grep 'sightglass-benchmark' | awk '{print $1}')
if [[ ! -z "$CONTAINERS" ]]; then
    (set -x; docker rm "$CONTAINERS")
fi
IMAGES=$(docker images --format '{{.Repository}}:{{.Tag}}' | grep 'sightglass-benchmark')
if [[ ! -z "$IMAGES" ]]; then
    (set -x; docker rmi "$IMAGES")
fi
