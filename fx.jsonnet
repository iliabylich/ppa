local configure = import 'scripts/base.jsonnet';

local version = '40.0.1';

configure(
  file=std.thisFile,
  name='fx',
  description='Terminal JSON viewer & processor',
  version=version,
  source={
    type: 'release',
    url: 'https://github.com/antonmedv/fx.git',
    rev: version,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: ['wget https://github.com/antonmedv/fx/releases/download/' + version + '/fx_linux_amd64 -O fx'],
    override_dh_auto_install: ['install -Dm0755 fx debian/fx/usr/bin/fx'],
  },
)
