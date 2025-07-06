#!/usr/bin/env bash

set -euo pipefail

PREFIX=" Package: "

for DEB in "$@"; do
    echo "Processing $DEB"
    PACKAGE_INFO="$(dpkg -I "$DEB")"
    PACKAGE_NAME="$(echo "$PACKAGE_INFO" | grep "$PREFIX" | sed "s/$PREFIX//g")"
    echo "Detected package name: $PACKAGE_NAME"

    if [[ "$DEB" != *"$PACKAGE_NAME"* ]]; then
        echo "Error: $DEB does not contain $PACKAGE_NAME as a substring"
    fi

    ASSETS_TO_REMOVE="$(gh release view latest --json assets -q '.assets[].name' | grep "$PACKAGE_NAME")"
    if [[ -n "$ASSETS_TO_REMOVE" ]]; then
        echo "Assets to remove:"
        echo "$ASSETS_TO_REMOVE"

        for ASSET in $ASSETS_TO_REMOVE; do
            gh release delete-asset latest "$ASSET" --yes
        done
    fi


    echo "Uploading $DEB to GitHub release..."
    gh release upload latest "$DEB"
    echo
done
