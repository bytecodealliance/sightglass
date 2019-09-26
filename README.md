
A containerized runner for WASM benchmarks. Included is a driver
to facilitate downloading the latest runtimes and preparing files for
execution. Sightglass is used as the harness for collecting and
dumping the benchmark results.

## Quick start

This version of sightglass adds a plugin framework that allows for seamless running of new vm and benchmark combinations via a python driver. In addition it encapsulates the entire framework inside a
container. To get started simply:
- build the docker image
- run the container via the python script linked at sg_container_runner.sh

Ex:
- ./sg_container_build.sh

and then:
 - ./sg_container_runner.sh --help

or (for example):
- ./sg_container_runner.sh -r wasmtime_app -p shootout<br>

Note, the sg_container_runner simply calls the python script sg_runner.py inside the container. Any modifications made to sg_runner.py will apply the next time sg_container_runner.sh is invoked.
