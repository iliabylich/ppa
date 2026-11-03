local configure = import 'scripts/base.jsonnet';

local version = '2.0.3';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/nm-mon/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='nm-mon',
  description='A simple NetworkManager monitoring service that sends data over UNIX socket',
  version=version,
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/nm-mon.git',
    rev: tag,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/nm-mon -O nm-mon',
      'wget ' + releaseUrl + '/nm-mon.service -O nm-mon.service',
      'wget ' + releaseUrl + '/nm-mon.socket -O nm-mon.socket',
    ],
    override_dh_auto_install: [
      'install -Dm0755 nm-mon debian/nm-mon/usr/bin/nm-mon',
      'install -Dm0644 nm-mon.service debian/nm-mon/usr/lib/systemd/system/nm-mon.service',
      'install -Dm0644 nm-mon.socket debian/nm-mon/usr/lib/systemd/system/nm-mon.socket',
    ],
  },
)
