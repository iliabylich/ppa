local configure = import '../scripts/base.jsonnet';
local version = '4.1';

configure(
  file=std.thisFile,
  name='fonts-inter-variable-nerd',
  description='The Inter font family (patched with NerdFont)',
  arch='all',
  version=version,
  install=[
    'wget',
    'unzip',
    'python3',
    'python3-fontforge',
    'parallel',
  ],
  source={
    type: 'release',
    url: 'https://github.com/rsms/inter.git',
    rev: 'v' + version,
  },
  dependencies=[],
  rules={
    override_dh_auto_build: [
      'wget https://github.com/rsms/inter/releases/download/v' + version + '/Inter-' + version + '.zip',
      'unzip -o Inter-' + version + '.zip',
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
