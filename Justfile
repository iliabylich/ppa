default:
    @just --list

clean:
    rm -f *.deb
    rm -rf deb-latest

container-rebuild:
    cargo build --release
    podman image rm -f ppa-builder
    podman build . --file Dockerfile --tag ppa-builder:latest --squash-all

container-sh:
    podman run --rm -it -v $PWD:/shared --entrypoint bash ppa-builder

run-in-container exe *command:
    podman run --rm \
        -t \
        -v "$PWD:/shared" \
        --entrypoint "/bin/{{exe}}" \
        ppa-builder \
        {{command}}

build config:
    @just run-in-container build {{config}}

parse config:
    @just run-in-container parse {{config}}

explain config:
    @just run-in-container explain {{config}}

gh-upload *args:
    ./scripts/gh-upload.sh {{args}}

deploy *args:
    ./scripts/deploy.sh {{args}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp

shellcheck:
    shellcheck -x **/*.sh

bump config:
    cargo run --bin bump -- {{config}}

check-updates:
    cargo run --bin check-updates -- $(find . -name '*.toml')
