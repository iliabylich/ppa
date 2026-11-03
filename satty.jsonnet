local configure = import 'scripts/base.jsonnet';
local version = '0.22.0';

configure(
  file=std.thisFile,
  name='satty',
  description='Satty - Modern Screenshot Annotation. A tool inspired by Swappy and Flameshot.',
  version=version,
  install=[
    'libgtk-4-dev',
    'libadwaita-1-dev',
    'libepoxy-dev',
    'libfontconfig-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/gabm/Satty.git',
    rev: 'v' + version,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: ['make build-release'],
    override_dh_auto_install: ['PREFIX=debian/satty/usr make install'],
  },
)
