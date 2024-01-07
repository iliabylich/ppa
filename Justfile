gpg-generate-key:
    gpg --batch --gen-key gpg/gpg-key.conf
    gpg --list-keys
gpg-export-key:
    gpg --armor --export "ibylich@gmail.com" > gpg/public.gpg

debian-rebuild:
    dpkg-scanpackages --multiversion . > Packages
    gzip -k -f Packages
    apt-ftparchive release . > Release
    gpg --default-key "ibylich@gmail.com" -abs -o - Release > Release.gpg
    gpg --default-key "ibylich@gmail.com" --clearsign -o - Release > InRelease
    git add InRelease Packages Packages.gz Release Release.gpg
debian-clean:
    rm -f InRelease Packages Packages.gz Release Release.gpg

download-latest repo:
    ./scripts/download-latest.sh "{{repo}}"
download-packages:
    ./scripts/download.sh

commit-and-push:
    git add .
    git commit --amend --no-edit
    git push --force
    ./scripts/wait-for-ci.sh

deploy:
    @just download-packages
    @just debian-rebuild
    @just commit-and-push
