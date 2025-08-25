#!/usr/bin/env bash

set -eux

apt update
apt -y upgrade

apt install --no-install-recommends -y ca-certificates
update-ca-certificates

apt install -y wget curl git debhelper pkg-config jq
apt install -y g++ meson cmake
apt install -y musl musl-dev
apt install -y gcc-15 g++-15

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --profile minimal -y

apt clean
rm -rf /var/lib/apt/lists/*
rm -rf /tmp/*

cd /root/.cargo/bin
curl -L https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-gnu.tgz | tar -xvzf -
