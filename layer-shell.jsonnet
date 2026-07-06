local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='layer-shell',
  description='A custom layer shell',
  arch='amd64',
  version='0.0.' + std.extVar('timestamp'),
  install=[
    'qt6-base-dev',
    'liblayershellqtinterface-dev',
    'libqtermwidget-dev',
    'sassc',
    'xxd',
    'meson',
    'liburing-dev',
    'libssl-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/layer-shell.git',
    rev: 'master',
  },
  scripts=[
    'wget https://github.com/mozilla/cbindgen/releases/download/0.28.0/cbindgen -O /bin/cbindgen',
    'chmod +x /bin/cbindgen',
  ],
  dependencies=[
    '${shlibs:Depends}',
    'fonts-inter-variable-nerd',
    'fonts-adwaita-mono-nerd',
  ],
  rules={
    '%': ['dh $@'],
    override_dh_auto_configure: ['dh_auto_configure -- --buildtype=release'],
  },
)
