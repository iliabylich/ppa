local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='kb-mod-monitor',
  description='Tiny Linux daemon that watches system input devices and exposes {Caps,Num} Lock state changes over a UNIX socket',
  arch='amd64',
  version='5.0.0',
  install=['libxkbcommon-dev'],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/kb-mod-monitor.git',
    rev: 'v5.0.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: [
      'install -Dm0755 target/release/kb-mod-monitor debian/kb-mod-monitor/usr/bin/kb-mod-monitor',
      'install -Dm0644 systemd/kb-mod-monitor.service debian/kb-mod-monitor/usr/lib/systemd/system/kb-mod-monitor.service',
      'install -Dm0644 systemd/kb-mod-monitor.socket debian/kb-mod-monitor/usr/lib/systemd/system/kb-mod-monitor.socket',
    ],
  },
)
