local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='tray-mon',
  description='A daemon service that triggers tray events over a UNIX socket',
  arch='amd64',
  version='1.0.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/tray-mon.git',
    rev: 'v1.0.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: [
      'install -Dm0755 target/release/tray-mon debian/tray-mon/usr/bin/tray-mon',
      'install -Dm0644 systemd/tray-mon.service debian/tray-mon/usr/lib/systemd/user/tray-mon.service',
      'install -Dm0644 systemd/tray-mon.socket debian/tray-mon/usr/lib/systemd/user/tray-mon.socket',
    ],
  },
)
