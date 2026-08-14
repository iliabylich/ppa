#!/usr/bin/env bash

set -euo pipefail
export PS4=$'\e[1;35m>>> \e[0m'
set -x
export DEB_BUILD_OPTIONS="parallel=$(nproc)"
export CARGO_TERM_COLOR="always"
export BAR="bar"
export FOO="foo"
export PATH="/foo/bin:/bar/bin:$PATH"
apt update
apt install --no-install-recommends --no-install-suggests -y foo-dev bar-dev
cargo binstall -y cargo-foo cargo-bar
mkdir -p /build
git clone https://github.com/test/repo.git --filter=blob:none --recursive --shallow-submodules --depth=1 -q --branch=v1.2.3 /build/testpackage-1.2.3
cd /build/testpackage-1.2.3
ls -l --color=always
echo foo
echo bar
mkdir -p debian
tee debian/changelog <<'EOF'
testpackage (1.2.3) unstable; urgency=low

  * Release

 -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000
EOF
tee debian/compat <<'EOF'
10
EOF
tee debian/control <<'EOF'
Source: testpackage
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: testpackage
Section: utils
Priority: extra
Architecture: all
Depends: ${shlibs:Depends},libfoo
Description: A test package
EOF
tee debian/rules <<'EOF'
#!/usr/bin/make -f
export DH_VERBOSE = 1

%:
	dh $@
bar:
	bar1
	bar2 --yes
foo:
	foo1
	foo2 --yes
override_dh_auto_test:
	@true
override_dh_strip:
	@true
EOF
chmod +x debian/rules
dh binary
ls -l /build --color=always
cp /build/testpackage_1.2.3_all.deb /shared/testpackage_1.2.3_all.deb
mkdir -p /shared/deb-latest
cp /build/testpackage_1.2.3_all.deb /shared/deb-latest/testpackage.deb
