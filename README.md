A containerized version of the database and server (webui) included under
sightglass's main branch. This container is intended to receive and serve results sent from the runner (see the runner branch).

## Quick start

Build and launch with
- ./sg_webui_run.sh

The default viewer port is at 3000.
The default history port is 8001.

TODO: Currently changing the port values in config.inc does not have impact on
the actual PORTS used by the viewer and history servers. They do serve to open
the appropriate corresponding (3000 and 8001) docker ports so their values are
used.
