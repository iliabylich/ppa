local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='matugen',
  description='A material you color generation tool',
  arch='amd64',
  version='4.1.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/InioX/matugen.git',
    rev: 'v4.1.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['CARGO_TERM_COLOR=always cargo build --release'],
    override_dh_auto_install: ['install -Dm0755 target/release/matugen debian/matugen/usr/bin/matugen'],
  },
)
