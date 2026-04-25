#!/usr/bin/env bash

set -ex

wasm-tools parse splay.wat -o splay.wasm
