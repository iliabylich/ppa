default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

container-rebuild:
    podman image rm -f ppa-builder
    podman build . --file Dockerfile --tag ppa-builder:latest --squash-all

container-sh:
    podman run --rm -it -v $PWD:/shared --entrypoint bash ppa-builder

_jsonnet config action:
    jsonnet {{config}} --ext-str timestamp=$(date +%s) --tla-str action={{action}} -S

build config:
    podman run --rm -t -v "$PWD:/shared" --entrypoint bash ppa-builder -c "$(just render {{config}}) | bash"

render config:
    @just _jsonnet {{config}} render

dump config:
    @just _jsonnet {{config}} dump

test:
    #!/usr/bin/env bash
    diff test/output.sh <(just render test/input.jsonnet)

push *args:
    ./scripts/deploy.sh {{args}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp

shellcheck:
    shellcheck -x **/*.sh

check-updates:
    #!/usr/bin/env bash
    set -euo pipefail

    find . \
        -path ./scripts -prune -o \
        -path ./test -prune -o \
        -type f \
        -name '*.jsonnet' \
        -print0 |
        xargs -0 -n 1 -P 8 bash -c '
            set -euo pipefail

            jsonnet "$1" --ext-str timestamp="$(date +%s)" --tla-str action=check-updates -S | bash
        ' _

reload-metapackage:
    sudo apt-mark auto $(jsonnet metapackage.jsonnet --tla-str action=print-dependencies -S)

remove-opt-packages:
    sudo apt autoremove -o Apt::AutoRemove::RecommendsImportant=false -o Apt::AutoRemove::SuggestsImportant=false
