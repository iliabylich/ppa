local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='pipewire-mon',
  description='A daemon service that triggers Pipewire volume events over UNIX socket',
  arch='amd64',
  version='2.0.0',
  install=[
    'libpipewire-0.3-dev',
    'libclang-rt-dev',
    'clang',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/pipewire-mon.git',
    rev: 'v2.0.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}', 'pipewire-bin'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: [
      'install -Dm0755 target/release/pipewire-mon debian/pipewire-mon/usr/bin/pipewire-mon',
      'install -Dm0644 systemd/pipewire-mon.service debian/pipewire-mon/usr/lib/systemd/user/pipewire-mon.service',
      'install -Dm0644 systemd/pipewire-mon.socket debian/pipewire-mon/usr/lib/systemd/user/pipewire-mon.socket',
      'install -Dm0644 data/silence.wav debian/pipewire-mon/usr/share/pipewire-mon/silence.wav',
    ],
  },
)
