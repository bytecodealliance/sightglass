#!/bin/bash

. "$(dirname ${BASH_SOURCE:-$0})/config.inc"
: "${SG_DOCKER_IMAGE_NAME:=sg_webui_image}"
: "${SG_DOCKER_CONTAINER_NAME:=sg_webui}"

if ! docker image inspect sg_webui_image > /dev/null; then
	${HOST_BASE_PREFIX}/sg_webui_build.sh
fi

echo "Checking for container ${SG_DOCKER_CONTAINER_NAME}"
if [[ $(docker ps --all -f "name=${SG_DOCKER_CONTAINER_NAME}" --format '{{.Names}}') == ${SG_DOCKER_CONTAINER_NAME} ]]; then
    if [ $(docker inspect -f '{{.State.Running}}' sg_webui) == 'false' ]; then
        echo "The container exists. Will docker start ${SG_DOCKER_CONTAINER_NAME}" >&2 
        docker start ${SG_DOCKER_CONTAINER_NAME} 
    else
        echo "The container is already running ... exiting." >&2 
    fi
else
    echo "Container does not exist. Will docker run ${SG_DOCKER_CONTAINER_NAME} based on image ${SG_DOCKER_IMAGE_NAME}"
    echo "History file at: ${HOST_BASE_PREFIX}/${SG_HISTORY_FILE}"

    if [[ ! -e ${HOST_BASE_PREFIX}/${SG_HISTORY_FILE} ]]; then
        cp ${HOST_BASE_PREFIX}/history.json ${HOST_BASE_PREFIX}/${SG_HISTORY_FILE}
    fi

    docker run -it --name=${SG_DOCKER_CONTAINER_NAME} --detach --volume  ${HOST_BASE_PREFIX}/${SG_HISTORY_FILE} \
    -p ${SG_VIEW_PORT}:${SG_VIEW_PORT} \
    -p ${SG_HISTORY_PORT}:${SG_HISTORY_PORT} \
    --mount type=bind,source=${HOST_BASE_PREFIX}/sg_webui_start.sh,target=/sg_webui_start.sh \
    --mount type=bind,source=${HOST_BASE_PREFIX}/sg_webui_stop.sh,target=/sg_webui_stop.sh \
    --mount type=bind,source=${HOST_BASE_PREFIX}/${SG_HISTORY_FILE},target=/webui/sg-history/history.json \
    ${SG_DOCKER_IMAGE_NAME} tail -f /dev/null 
fi

echo "Launch Webui"
docker exec ${SG_DOCKER_CONTAINER_NAME} /sg_webui_start.sh ${SG_HOSTNAME} ${SG_VIEW_PORT} ${SG_HISTORY_PORT} ${SG_HISTORY_LOCATION} &

