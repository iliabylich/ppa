#!/usr/bin/env bash

set -euo pipefail

TOKEN="$(cat config.toml | tomlq -r ".token")"
URL="$(cat config.toml | tomlq -r ".url")"

echo "Uploading to $URL using token $TOKEN"

curlf() {
    RESPONSE_FILE="$(mktemp)"
    HTTP_CODE="$(curl --silent --output $RESPONSE_FILE --write-out "%{http_code}" "$@")"
    RESPONSE="$(cat "$RESPONSE_FILE")"
    rm $RESPONSE_FILE

    echo "$HTTP_CODE - $RESPONSE"

    if [[ ${HTTP_CODE} -lt 200 || ${HTTP_CODE} -gt 299 ]] ; then
        return 22
    fi
}


for DEB in "$@"; do
    echo "Uploading $DEB"

    curlf -F "$DEB=@$DEB" -H "Token: $TOKEN" "$URL"
done
