local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='bzmenu',
  description='Launcher-driven Wi-Fi manager for Linux',
  arch='amd64',
  version='0.4.0',
  install=['libdbus-1-dev'],
  source={
    type: 'git',
    url: 'https://github.com/e-tho/bzmenu.git',
    rev: 'v0.4.0',
  },
  scripts=[],
  dependencies=[
    '${shlibs:Depends}',
    'bluez',
  ],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/bzmenu debian/bzmenu/usr/bin/bzmenu'],
  },
)
