local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fonts-adwaita-mono-nerd',
  description='The Adwaita Mono font family (patched with NerdFont)',
  arch='all',
  version='3.4.0',
  install=[
    'wget',
    'unzip',
  ],
  source={
    type: 'git',
    url: 'https://github.com/ryanoasis/nerd-fonts.git',
    rev: 'v3.4.0',
  },
  scripts=[],
  dependencies=[],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.4.0/AdwaitaMono.zip',
      'unzip -o AdwaitaMono.zip',
    ],
    override_dh_auto_install: [
      'mkdir -p debian/fonts-adwaita-mono-nerd/usr/share/fonts/adwaita-mono-nerd',
      'install -m644 *.ttf debian/fonts-adwaita-mono-nerd/usr/share/fonts/adwaita-mono-nerd/',
    ],
  },
)
