#!/bin/bash

. "$(dirname ${BASH_SOURCE:-$0})/config.inc"
: "${SG_DOCKER_IMAGE_NAME:=sg_runner_image}"
: "${SG_DOCKER_CONTAINER_NAME:=sg_runner}"

if ! docker image inspect sg_runner_image > /dev/null; then
	${HOST_BASE_PREFIX}/sg_image_build.sh
fi

if [[ $(docker ps --all -f "name=${SG_DOCKER_CONTAINER_NAME}" --format '{{.Names}}') == ${SG_DOCKER_CONTAINER_NAME} ]]; then
    if [ $(docker inspect -f '{{.State.Running}}' sg_runner) == 'false' ]; then
        docker container rm ${SG_DOCKER_CONTAINER_NAME} > /dev/null
    else
        echo "The container is currently running ... exiting." >&2
        exit
    fi
fi

if [ -z "$SG_JUST_ENTER" ];  then
    docker run --net=host -it --name=${SG_DOCKER_CONTAINER_NAME} --volume  ${HOST_BASE_PREFIX}/../:/sightglass_runner/ \
				-w /sightglass_runner/webui_runner \
        ${SG_DOCKER_IMAGE_NAME}  python3.6 /sightglass_runner/webui_runner/sg_runner.py $@
else
    docker run --net=host -it --name="${SG_DOCKER_CONTAINER_NAME}" --volume  ${HOST_BASE_PREFIX}/../:/sightglass_runner/ \
        ${SG_DOCKER_IMAGE_NAME} /bin/bash
fi
