#!/usr/bin/env bash

set -eu

GPG_URL="https://iliabylich.github.io/ppa/gpg/public.gpg"
GPG_PATH="/etc/apt/trusted.gpg.d/iliabylich_ppa.gpg"

SOURCE_URL="https://iliabylich.github.io/ppa/apt/ppa.source"
SOURCE_PATH="/etc/apt/sources.list.d/iliabylich_ppa.sources"

curl --silent "$GPG_URL" | gpg --dearmor | sudo tee "$GPG_PATH" > /dev/null
curl --silent "$SOURCE_URL" | sudo tee "$SOURCE_PATH" > /dev/null
