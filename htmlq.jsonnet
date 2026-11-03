local configure = import 'scripts/base.jsonnet';
local version = '0.4.0';

configure(
  file=std.thisFile,
  name='htmlq',
  description='Like jq, but for HTML.',
  version=version,
  source={
    type: 'git',
    url: 'https://github.com/mgdm/htmlq.git',
    rev: 'v' + version,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: ['cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/htmlq debian/htmlq/usr/bin/htmlq'],
  },
)
