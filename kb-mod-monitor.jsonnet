local configure = import 'scripts/base.jsonnet';

local version = '5.0.2';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/kb-mod-monitor/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='kb-mod-monitor',
  description='Tiny Linux daemon that watches system input devices and exposes {Caps,Num} Lock state changes over a UNIX socket',
  version=version,
  install=['libxkbcommon0'],
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/kb-mod-monitor.git',
    rev: tag,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/kb-mod-monitor -O kb-mod-monitor',
      'wget ' + releaseUrl + '/kb-mod-monitor.service -O kb-mod-monitor.service',
      'wget ' + releaseUrl + '/kb-mod-monitor.socket -O kb-mod-monitor.socket',
    ],
    override_dh_auto_install: [
      'install -Dm0755 kb-mod-monitor debian/kb-mod-monitor/usr/bin/kb-mod-monitor',
      'install -Dm0644 kb-mod-monitor.service debian/kb-mod-monitor/usr/lib/systemd/system/kb-mod-monitor.service',
      'install -Dm0644 kb-mod-monitor.socket debian/kb-mod-monitor/usr/lib/systemd/system/kb-mod-monitor.socket',
    ],
  },
)
