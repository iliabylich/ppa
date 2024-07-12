#!/usr/bin/env bash

set -eux

PPA_URL="https://iliabylich.github.io/ppa"
PUBLIC_KEY_URL="$PPA_URL/iliabylich_ppa.gpg"
PUBLIC_KEY_PATH="/etc/apt/trusted.gpg.d/iliabylich_ppa.gpg"

curl -s --compressed "$PUBLIC_KEY_URL" | gpg --dearmor | sudo tee "$PUBLIC_KEY_PATH" > /dev/null

SOURCE_LIST="deb [signed-by=$PUBLIC_KEY_PATH] $PPA_URL ./"
SOURCE_PATH="/etc/apt/sources.list.d/iliabylich_ppa.list"
echo "$SOURCE_LIST" | sudo tee "$SOURCE_PATH" > /dev/null
