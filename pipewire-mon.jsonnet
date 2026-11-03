local configure = import 'scripts/base.jsonnet';

local version = '2.0.1';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/pipewire-mon/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='pipewire-mon',
  description='A daemon service that triggers Pipewire volume events over UNIX socket',
  version=version,
  install=['libpipewire-0.3-0t64'],
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/pipewire-mon.git',
    rev: tag,
  },
  dependencies=['${shlibs:Depends}', 'pipewire-bin'],
  rules={
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/pipewire-mon -O pipewire-mon',
      'wget ' + releaseUrl + '/pipewire-mon.service -O pipewire-mon.service',
      'wget ' + releaseUrl + '/pipewire-mon.socket -O pipewire-mon.socket',
      'wget ' + releaseUrl + '/silence.wav -O silence.wav',
    ],
    override_dh_auto_install: [
      'install -Dm0755 pipewire-mon debian/pipewire-mon/usr/bin/pipewire-mon',
      'install -Dm0644 pipewire-mon.service debian/pipewire-mon/usr/lib/systemd/user/pipewire-mon.service',
      'install -Dm0644 pipewire-mon.socket debian/pipewire-mon/usr/lib/systemd/user/pipewire-mon.socket',
      'install -Dm0644 silence.wav debian/pipewire-mon/usr/share/pipewire-mon/silence.wav',
    ],
  },
)
