# Sightglass

A benchmark suite and tool to compare different implementations of the same primitives. It contains
several parts:
 - a legacy test runner, described [here](docs/legacy.md) and implemented in [src](src) with
   benchmarks included in [benchmarks](benchmarks)
 - a new version of the test runner with different benchmark conventions, described
   [here](docs/next.md) and implemented in [crates](crates) with benchmarks included in
   [benchmarks-next](benchmarks-next)
 - a harness using the legacy test runner that measures results in different engines (e.g. lucet,
   wasmtime, wamr, [etc.](webui_runner/plugs)) with benchmarks in
   [webui_runner/benchmarks](webui_runner/benchmarks)
 - a web application for displaying results stored in the [sg-history](webui/sg-history) web service
   and displayed in teh [sg-view](webui/sg-history) user interface.

> Note: this repository is under active development--folders may be re-organized and APIs may
> change!
