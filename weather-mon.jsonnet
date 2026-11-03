local configure = import 'scripts/base.jsonnet';

local version = '2.0.2';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/weather-mon/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='weather-mon',
  description='A daemon service that fetches weather and shares it over a UNIX socket',
  version=version,
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/weather-mon.git',
    rev: tag,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/weather-mon -O weather-mon',
      'wget ' + releaseUrl + '/weather-mon.service -O weather-mon.service',
      'wget ' + releaseUrl + '/weather-mon.socket -O weather-mon.socket',
    ],
    override_dh_auto_install: [
      'install -Dm0755 weather-mon debian/weather-mon/usr/bin/weather-mon',
      'install -Dm0644 weather-mon.service debian/weather-mon/usr/lib/systemd/user/weather-mon.service',
      'install -Dm0644 weather-mon.socket debian/weather-mon/usr/lib/systemd/user/weather-mon.socket',
    ],
  },
)
