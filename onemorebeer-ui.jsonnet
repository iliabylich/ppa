local configure = import 'scripts/base.jsonnet';

local version = '1.0.0';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/onemorebeer-ui/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='onemorebeer-ui',
  description='onemorebeer ui',
  arch='amd64',
  version=version,
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/onemorebeer-ui.git',
    rev: tag,
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/onemorebeer-ui -O onemorebeer-ui',
      'wget ' + releaseUrl + '/onemorebeer-ui.service -O onemorebeer-ui.service',
    ],
    override_dh_auto_install: [
      'install -Dm0755 onemorebeer-ui debian/onemorebeer-ui/usr/bin/onemorebeer-ui',
      'install -Dm0644 onemorebeer-ui.service debian/onemorebeer-ui/usr/lib/systemd/system/onemorebeer-ui.service',
    ],
  },
)
