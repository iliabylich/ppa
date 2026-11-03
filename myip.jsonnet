local configure = import 'scripts/base.jsonnet';

local version = '1.0.0';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/myip/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='myip',
  description='a tiny service that returns client\'s IP',
  version=version,
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/myip.git',
    rev: tag,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/myip -O myip',
      'wget ' + releaseUrl + '/myip.service -O myip.service',
    ],
    override_dh_auto_install: [
      'install -Dm0755 myip debian/myip/usr/bin/myip',
      'install -Dm0644 myip.service debian/myip/usr/lib/systemd/system/myip.service',
    ],
  },
)
