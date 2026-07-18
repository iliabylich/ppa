local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='weather-mon',
  description='A daemon service that fetches weather and shares it over a UNIX socket',
  arch='amd64',
  version='1.0.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/weather-mon.git',
    rev: 'v1.0.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: [
      'install -Dm0755 target/release/weather-mon debian/weather-mon/usr/bin/weather-mon',
      'install -Dm0644 systemd/weather-mon.service debian/weather-mon/usr/lib/systemd/user/weather-mon.service',
      'install -Dm0644 systemd/weather-mon.socket debian/weather-mon/usr/lib/systemd/user/weather-mon.socket',
    ],
  },
)
