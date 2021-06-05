sightglass-result-server
========================

This crate (previously named `sg-history`), records benchmark results as HTTP resource for
consumption from a web application.

### Run

Run the web service directly using Cargo:

```
cargo run -p sightglass-result-server
```

Alternately, the Docker image will provide the necessary library environment for building and
running the web service if Cargo/Rust are not available:

```
docker build -t sightglass-result-server:latest .
docker run --name sightglass-result-server --p 8001:8001 --rm --detach sightglass-result-server:latest
# use `--volume ...` to share the /opt/sightglass/history.json file with the host
```

Configure the result server using the following environment variables:
 - `LISTEN_ADDRESS`: configure the IP and port the server binds to; by default, the server will
   attempt to bind to `0.0.0.0:8001`.
 - `HISTORY_PATH`: configure the path to the file containing the history results; by default, this
   is `./history.json`.

Note: the Docker environment can be modified with [`docker run --env
VAR=value ...`](https://docs.docker.com/engine/reference/commandline/run/#set-environment-variables--e---env---env-file).

### Use

The web service has two main endpoints:
 - `POST /submit` accepts correctly-formatted JSON results (see
   [`test/fixtures`][../../test/fixtures], e.g.) and adds them to the database (currently just a
   single file).
 - `GET /history` returns all previously-submitted results as JSON. 

Note: the data formats used for these endpoints are not yet stable and will likely change as this
crate evolves (e.g. to add result filtering or additional fields)/.
