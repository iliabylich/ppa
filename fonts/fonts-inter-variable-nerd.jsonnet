local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fonts-inter-variable-nerd',
  description='The Inter font family (patched with NerdFont)',
  arch='all',
  version='4.1',
  install=[
    'wget',
    'unzip',
    'python3',
    'python3-fontforge',
    'parallel',
  ],
  source={
    type: 'git',
    url: 'https://github.com/rsms/inter.git',
    rev: 'v4.1',
  },
  scripts=[],
  dependencies=[],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget https://github.com/rsms/inter/releases/download/v4.1/Inter-4.1.zip',
      'unzip -o Inter-4.1.zip',
      'wget https://github.com/ryanoasis/nerd-fonts/releases/latest/download/FontPatcher.zip',
      'unzip -o FontPatcher.zip',
      '/shared/fonts/patch-inter.sh',
    ],
    override_dh_auto_install: [
      'mkdir -p debian/fonts-inter-variable-nerd/usr/share/fonts/inter-nerd',
      'install -m644 patched/*.otf debian/fonts-inter-variable-nerd/usr/share/fonts/inter-nerd/',
    ],
  },
)
