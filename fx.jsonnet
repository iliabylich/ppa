local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fx',
  description='Terminal JSON viewer & processor',
  arch='amd64',
  version='39.2.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/antonmedv/fx.git',
    rev: '39.2.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['wget https://github.com/antonmedv/fx/releases/download/39.2.0/fx_linux_amd64 -O fx'],
    override_dh_auto_install: ['install -Dm0755 fx debian/fx/usr/bin/fx'],
  },
)
