#!/usr/bin/env bash

set -ex

wasm-tools parse sqlite3.wat -o sqlite3.wasm
