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

container-run command config_path:
    podman run --rm \
        -e BASE_CONFIGS_DIR="/shared" \
        -e CONFIG_PATH="{{ config_path }}" \
        -t \
        -v "$PWD:/shared" \
        --entrypoint "/bin/build-deb-package" \
        ppa-builder \
        {{command}}

build config_path:
    @just container-run build {{config_path}}

parse config_path:
    @just container-run parse {{config_path}}

explain config_path:
    @just container-run explain {{config_path}}

gh-upload *args:
    ./scripts/gh-upload.sh {{args}}

deploy *args:
    ./scripts/deploy.sh {{args}}

unpack debfile:
    mkdir -p tmp
    dpkg-deb -R {{debfile}} tmp

shellcheck:
    shellcheck -x **/*.sh

bump config_path:
    BASE_CONFIGS_DIR=$PWD CONFIG_PATH={{config_path}} cargo run -- bump-version-trailer
