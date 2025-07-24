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

run-in-container *command:
    podman run --rm \
        -t \
        -v "$PWD:/shared" \
        --entrypoint "/bin/build-deb-package" \
        ppa-builder \
        {{command}}

run-locally *command:
    cargo run -- {{command}}

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
    @just run-locally bump-version-trailer {{config}}

check-updates:
    @just run-locally check-updates $(find . -name '*.toml')
