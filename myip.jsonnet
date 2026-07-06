local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='myip',
  description='a tiny service that returns client\'s IP',
  arch='amd64',
  version='0.0.' + std.extVar('timestamp'),
  install=[
    'musl',
    'musl-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/myip.git',
    rev: 'master',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'rustup target add x86_64-unknown-linux-musl',
      'cargo build --release --target=x86_64-unknown-linux-musl',
    ],
    override_dh_auto_install: [
      'install -Dm0755 target/x86_64-unknown-linux-musl/release/myip debian/myip/usr/bin/myip',
      'install -Dm0644 debian/myip.service debian/myip/usr/lib/systemd/system/myip.service',
    ],
  },
)
