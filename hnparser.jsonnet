local configure = import 'scripts/base.jsonnet';

local version = '1.0.0';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/hn-parser/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='hnparser',
  description='HN parser',
  arch='amd64',
  version=version,
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/hn-parser.git',
    rev: tag,
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/hnparser -O hnparser',
      'wget ' + releaseUrl + '/hnparser.service -O hnparser.service',
    ],
    override_dh_auto_install: [
      'install -Dm0755 hnparser debian/hnparser/usr/bin/hnparser',
      'install -Dm0644 hnparser.service debian/hnparser/usr/lib/systemd/system/hnparser.service',
    ],
  },
)
