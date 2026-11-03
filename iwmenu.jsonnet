local configure = import 'scripts/base.jsonnet';
local version = '0.4.0';

configure(
  file=std.thisFile,
  name='iwmenu',
  description='Launcher-driven Wi-Fi manager for Linux',
  version=version,
  source={
    type: 'git',
    url: 'https://github.com/e-tho/iwmenu.git',
    rev: 'v' + version,
  },
  dependencies=[
    '${shlibs:Depends}',
    'iwd',
  ],
  rules={
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/iwmenu debian/iwmenu/usr/bin/iwmenu'],
  },
)
