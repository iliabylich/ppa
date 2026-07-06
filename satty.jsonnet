local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='satty',
  description='Satty - Modern Screenshot Annotation. A tool inspired by Swappy and Flameshot.',
  arch='amd64',
  version='0.20.1',
  install=[
    'libgtk-4-dev',
    'libadwaita-1-dev',
    'libepoxy-dev',
    'libfontconfig-dev',
    'just',
  ],
  source={
    type: 'git',
    url: 'https://github.com/gabm/Satty.git',
    rev: 'v0.20.1',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['make build-release'],
    override_dh_auto_install: ['PREFIX=debian/satty/usr make install'],
  },
)
