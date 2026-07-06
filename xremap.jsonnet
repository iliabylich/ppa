local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='xremap',
  description='Key remapper for X11 and Wayland',
  arch='amd64',
  version='0.15.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/xremap/xremap.git',
    rev: 'v0.15.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release --features hypr'],
    override_dh_auto_install: ['install -Dm0755 target/release/xremap debian/xremap/usr/bin/xremap'],
  },
)
