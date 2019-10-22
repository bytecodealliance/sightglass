#! /bin/sh

HOST_BASE_PREFIX=${HOST_BASE_PREFIX:-$(cd "$(dirname ${BASH_SOURCE:-$0})" && pwd)}
SG_DOCKER_IMAGE_NAME="sg_webui_image"
SG_DOCKER_CONTAINER_NAME="sg_webui"

# TODO: PORTS are currently hardcoded in the docker image
# Any change here must be consistent with the docker image
SG_VIEW_PORT=3000
SG_HISTORY_PORT=8001
SG_HOSTNAME=localhost

# History file location is relative to HOST_BASE_PREFIX
SG_HISTORY_FILE="history.json"
