local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='pipewire-dbus',
  description='A daemon service that triggers Pipewire volume events over DBus',
  arch='amd64',
  version='3.0.4',
  install=[
    'libpipewire-0.3-dev',
    'libclang-rt-dev',
    'clang',
    'meson',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/pipewire-dbus.git',
    rev: 'v3.0.4',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_configure: ['dh_auto_configure -- --buildtype=release'],
  },
)
