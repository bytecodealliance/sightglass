#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi
# Install OpenVINO and set it up.
wget https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB
sudo apt-key add GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB
echo "deb https://apt.repos.intel.com/openvino/2022 bionic main" | sudo tee /etc/apt/sources.list.d/intel-openvino-2022.list
sudo apt update
sudo apt install openvino
source /opt/intel/openvino_2022/bin/setupvars.sh
ln -s /opt/intel/openvino_2022 /opt/intel/openvino
