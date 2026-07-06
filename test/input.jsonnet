local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='testpackage',
  description='A test package',
  arch='all',
  version='1.2.3',
  env={
    FOO: 'foo',
    BAR: 'bar',
  },
  path=['/foo/bin', '/bar/bin'],
  install=['foo-dev', 'bar-dev'],
  binstall=['cargo-foo', 'cargo-bar'],
  source={
    type: 'git',
    url: 'https://github.com/test/repo.git',
    rev: 'v1.2.3',
  },
  scripts=['echo foo', 'echo bar'],
  dependencies=['${shlibs:Depends}', 'libfoo'],
  rules={
    foo: ['foo1', 'foo2 --yes'],
    '%': ['dh $@'],
    bar: ['bar1', 'bar2 --yes'],
  },
)
