#!/usr/bin/env bash

set -euo pipefail

SHA="$(git rev-parse HEAD)"

echo "Waiting for run on $SHA to appear..."

while : ; do
    RUN_ID="$(gh run list --commit "$SHA" --limit 1 --json "databaseId" --jq ".[0].databaseId")"

    if [ ! -z "$RUN_ID" ]; then
        break
    fi
done

echo "Run ID: $RUN_ID"
gh run watch "$RUN_ID"

notify-send "GitHub Pages pages have been updated"
