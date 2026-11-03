#!/usr/bin/env bash

set -euxo pipefail

apt-get update
apt-get -y upgrade

apt-get install -y wget curl git debhelper pkg-config jq ca-certificates
apt-get autoremove -y

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --profile minimal -y

apt-get clean
rm -rf /var/lib/apt/lists/*
rm -rf /tmp/*
