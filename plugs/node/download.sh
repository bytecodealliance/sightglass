#!/bin/bash

# Use NVM to get latest copy
source ~/.nvm/nvm.sh && nvm install node
node_location=$(which node)
echo "Node Location: ${node_location}"
mkdir -p build; cd build
cp ${node_location} ./

# Downloading code but not building
# Downloading allows to get commit id, message, etc
rm -rf nodejs
git clone https://github.com/nodejs/node.git nodejs
tag_name=$(./node --version)
cd nodejs
git checkout tags/${tag_name}
cd ../../