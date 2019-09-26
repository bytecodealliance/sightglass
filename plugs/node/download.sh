#!/bin/bash

source ~/.nvm/nvm.sh && nvm install node
node_location=$(which node)
echo "Node Location: ${node_location}"
mkdir -p build; cd build
cp ${node_location} ./