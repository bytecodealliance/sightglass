#!/bin/bash

. "$(dirname ${BASH_SOURCE:-$0})/config.inc"
: "${SG_DOCKER_IMAGE_NAME:=sg_webui_image}"

echo "Build $SG_DOCKER_IMAGE_NAME"

if docker image inspect $SG_DOCKER_IMAGE_NAME > /dev/null; then
        if [ -z "$SG_WEBUI_FORCE_REBUILD" ]; then
                echo "A sg_webui image is already present"
                echo "Hit Ctrl-C right now if you don't want to rebuild it"
                echo "or skip this wait by setting the SG_WEBUI_FORCE_REBUILD variable"
                sleep 6
        fi
fi

docker build -t ${SG_DOCKER_IMAGE_NAME} .
