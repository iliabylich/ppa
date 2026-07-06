local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='iwmenu',
  description='Launcher-driven Wi-Fi manager for Linux',
  arch='amd64',
  version='0.4.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/e-tho/iwmenu.git',
    rev: 'v0.4.0',
  },
  scripts=[],
  dependencies=[
    '${shlibs:Depends}',
    'iwd',
  ],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/iwmenu debian/iwmenu/usr/bin/iwmenu'],
  },
)
