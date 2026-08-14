local configure = import 'scripts/base.jsonnet';
local version = '0.15.10';

configure(
  file=std.thisFile,
  name='xremap',
  description='Key remapper for X11 and Wayland',
  arch='amd64',
  version=version,
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/xremap/xremap.git',
    rev: 'v' + version,
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release --features hypr'],
    override_dh_auto_install: ['install -Dm0755 target/release/xremap debian/xremap/usr/bin/xremap'],
  },
)
