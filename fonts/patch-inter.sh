#!/usr/bin/env bash

set -euxo pipefail

parallel --halt now,fail=1 -j "$(nproc)" fontforge -script font-patcher --variable-width-glyphs -q -c {} -out ./patched &> /dev/null ::: extras/otf/*.otf
