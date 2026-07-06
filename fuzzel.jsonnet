local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fuzzel',
  description='App launcher and fuzzy finder for Wayland, inspired by rofi(1) and dmenu(1).',
  arch='amd64',
  version='1.14.1',
  install=[
    'libfontconfig-dev',
    'libpixman-1-dev',
    'libcairo2-dev',
    'wayland-protocols',
    'libwayland-dev',
    'libxkbcommon-dev',
    'librsvg2-dev',
    'scdoc',
  ],
  source={
    type: 'git',
    url: 'https://codeberg.org/dnkl/fuzzel.git',
    rev: '1.14.1',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_configure: ['dh_auto_configure -- --wrap-mode=default --buildtype=release -Denable-cairo=enabled -Dpng-backend=libpng -Dsvg-backend=librsvg'],
  },
)
