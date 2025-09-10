#!/usr/bin/env bash

set -eux

apt update
apt -y upgrade

apt install -y wget curl git debhelper pkg-config jq g++ meson cmake musl musl-dev
apt autoremove -y

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --profile minimal -y

apt clean
rm -rf /var/lib/apt/lists/*
rm -rf /tmp/*

cd /root/.cargo/bin
curl -L https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-gnu.tgz | tar -xvzf -
