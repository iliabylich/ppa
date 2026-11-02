local configure = import 'scripts/base.jsonnet';

local version = '1.0.4';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/tray-mon/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='tray-mon',
  description='A daemon service that triggers tray events over a UNIX socket',
  arch='amd64',
  version=version,
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/tray-mon.git',
    rev: tag,
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/tray-mon -O tray-mon',
      'wget ' + releaseUrl + '/tray-mon.service -O tray-mon.service',
      'wget ' + releaseUrl + '/tray-mon.socket -O tray-mon.socket',
    ],
    override_dh_auto_install: [
      'install -Dm0755 tray-mon debian/tray-mon/usr/bin/tray-mon',
      'install -Dm0644 tray-mon.service debian/tray-mon/usr/lib/systemd/user/tray-mon.service',
      'install -Dm0644 tray-mon.socket debian/tray-mon/usr/lib/systemd/user/tray-mon.socket',
    ],
  },
)
