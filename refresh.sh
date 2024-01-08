#!/usr/bin/env bash

set -eux

dpkg-scanpackages --multiversion . > Packages
gzip -k -f Packages

apt-ftparchive release . > Release

gpg --default-key "ibylich@gmail.com" -abs -o - Release > Release.gpg
gpg --default-key "ibylich@gmail.com" --clearsign -o - Release > InRelease
