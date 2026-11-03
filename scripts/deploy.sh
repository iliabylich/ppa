#!/usr/bin/env bash

set -euo pipefail

TOKEN="$(tomlq -r .token config.toml)"
URL="$(tomlq -r .url config.toml)"

echo "Uploading to $URL using token $TOKEN"

for DEB in "$@"; do
    echo "Uploading $DEB"
    curl --fail-with-body -sS -w ' (HTTP %{http_code})\n' -F "$DEB=@$DEB" -H "Token: $TOKEN" "$URL"
done
