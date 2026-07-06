local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='htmlq',
  description='Like jq, but for HTML.',
  arch='amd64',
  version='0.4.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/mgdm/htmlq.git',
    rev: 'v0.4.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['CARGO_TERM_COLOR=always cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/htmlq debian/htmlq/usr/bin/htmlq'],
  },
)
