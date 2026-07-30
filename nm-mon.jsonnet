local configure = import 'scripts/base.jsonnet';

local version = '2.0.1';

configure(
  file=std.thisFile,
  name='nm-mon',
  description='A simple NetworkManager monitoring service that sends data over UNIX socket',
  arch='amd64',
  version=version,
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/nm-mon.git',
    rev: 'v' + version,
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: [
      'install -Dm0755 target/release/nm-mon debian/nm-mon/usr/bin/nm-mon',
      'install -Dm0644 systemd/nm-mon.service debian/nm-mon/usr/lib/systemd/system/nm-mon.service',
      'install -Dm0644 systemd/nm-mon.socket debian/nm-mon/usr/lib/systemd/system/nm-mon.socket',
    ],
  },
)
