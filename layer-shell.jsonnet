local configure = import 'scripts/base.jsonnet';

local version = '1.0.0';
local tag = 'v' + version;
local releaseUrl = 'https://github.com/iliabylich/layer-shell/releases/download/' + tag;

configure(
  file=std.thisFile,
  name='layer-shell',
  description='A custom layer shell',
  version=version,
  install=[
    'libqt6core6t64',
    'libqt6gui6',
    'libqt6widgets6',
    'liblayershellqtinterface6',
    'libqtermwidget6-2',
  ],
  source={
    type: 'release',
    url: 'https://github.com/iliabylich/layer-shell.git',
    rev: tag,
  },
  dependencies=[
    '${shlibs:Depends}',
    'fonts-inter-variable-nerd',
    'fonts-adwaita-mono-nerd',
  ],
  rules={
    override_dh_auto_configure: ['@true'],
    override_dh_auto_build: [
      'wget ' + releaseUrl + '/layer-shell -O layer-shell',
      'wget ' + releaseUrl + '/layer-shell.service -O layer-shell.service',
    ],
    override_dh_auto_install: [
      'install -Dm0755 layer-shell debian/layer-shell/usr/bin/layer-shell',
      'install -Dm0644 layer-shell.service debian/layer-shell/usr/lib/systemd/user/layer-shell.service',
    ],
  },
)
