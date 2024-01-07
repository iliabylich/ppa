#!/usr/bin/env bash

set -eu

CONFIG_PATH="$1"
RELEASE_NAME=$(echo "${CONFIG_PATH%%.*}" | tr "/" "-")

echo "$RELEASE_NAME"

if ! gh release view "$RELEASE_NAME" &>/dev/null; then
    echo "Creating release $RELEASE_NAME"
    gh release create "$RELEASE_NAME" --title "$RELEASE_NAME" --notes "Automated release for $RELEASE_NAME"
else
    echo "Release $RELEASE_NAME already exists, skipping"
fi

ASSETS=$(gh release view "$RELEASE_NAME" --json "assets" --jq ".assets[] | .name")

for ASSET in $ASSETS; do
    echo "Deleting $ASSET..."
    gh release delete-asset "$RELEASE_NAME" "$ASSET" -y
done

for DEB in $(ls *.deb); do
    echo "Uploading $DEB..."
    gh release upload "$RELEASE_NAME" "$DEB"
done
