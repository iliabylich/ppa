local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='hnparser',
  description='HN parser',
  arch='amd64',
  version='0.0.' + std.extVar('timestamp'),
  install=[
    'just',
    'musl',
    'musl-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/hn-parser.git',
    rev: 'master',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'mise x -- npm install',
      'mise x -- npx @tailwindcss/cli -i templates/index.css -o ./templates/output.css --minify',
      'rustup target add x86_64-unknown-linux-musl',
      'cargo build --release --target=x86_64-unknown-linux-musl',
    ],
    override_dh_auto_install: [
      'install -Dm0755 target/x86_64-unknown-linux-musl/release/hnparser debian/hnparser/usr/bin/hnparser',
      'install -Dm0644 debian/hnparser.service debian/hnparser/usr/lib/systemd/system/hnparser.service',
    ],
  },
)
