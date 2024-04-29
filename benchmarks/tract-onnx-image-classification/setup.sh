#!/bin/bash

set -euxo pipefail

MODEL_URL="https://www.github.com/onnx/models/raw/main/validated/vision/classification/mobilenet/model/mobilenetv2-7.onnx?download=:"
MODEL_FILE="assets/mobilenetv2-7.onnx"

mkdir -p assets

if [ ! -f $MODEL_FILE ]; then
	echo "Downloading model to $MODEL_FILE"

	if which wget >/dev/null ; then
	wget $MODEL_URL -O $MODEL_FILE
	elif which curl >/dev/null ; then
	curl -vL $MODEL_URL -o $MODEL_FILE
	else
	echo "Couldn't find wget or curl."
	echo "Please download manually from \"$MODEL_URL\" and save the file in $MODEL_FILE."
	fi
fi
