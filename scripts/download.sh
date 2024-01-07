#!/usr/bin/env bash

set -euo pipefail

rm -f *.deb

RELEASES="$(gh release list --json "name" --jq ".[].name" | sort)"

echo "$RELEASES" | parallel -j20 --halt now,fail=1 --line-buffer '
  TAG={}
  gh release view "$TAG" --json assets --jq ".assets[] | \"$TAG\t\(.name)\""
' | parallel -j20 --bar --colsep '\t' --halt now,fail=1 '
  echo "Downloading: {2} from {1}"
  gh release download {1} --pattern {2} --clobber
'
