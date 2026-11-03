local configure = import 'scripts/base.jsonnet';
local version = '0.4.0';

configure(
  file=std.thisFile,
  name='bzmenu',
  description='Launcher-driven Bluetooth manager for Linux',
  version=version,
  install=['libdbus-1-dev'],
  source={
    type: 'git',
    url: 'https://github.com/e-tho/bzmenu.git',
    rev: 'v' + version,
  },
  dependencies=[
    '${shlibs:Depends}',
    'bluez',
  ],
  rules={
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/bzmenu debian/bzmenu/usr/bin/bzmenu'],
  },
)
