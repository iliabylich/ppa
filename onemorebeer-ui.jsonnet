local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='onemorebeer-ui',
  description='onemorebeer ui',
  arch='amd64',
  version='0.0.' + std.extVar('timestamp'),
  install=[
    'just',
    'musl',
    'musl-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/onemorebeer-ui.git',
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
      'install -Dm0755 target/x86_64-unknown-linux-musl/release/onemorebeer-ui debian/onemorebeer-ui/usr/bin/onemorebeer-ui',
      'install -Dm0644 debian/onemorebeer-ui.service debian/onemorebeer-ui/usr/lib/systemd/system/onemorebeer-ui.service',
    ],
  },
)
