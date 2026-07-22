local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fonts-adwaita-mono',
  description='The Adwaita Mono font family',
  arch='all',
  version='50.0',
  source={
    type: 'git',
    url: 'https://gitlab.gnome.org/GNOME/adwaita-fonts.git',
    rev: '50.0',
  },
  scripts=[],
  dependencies=[],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      '@true',
    ],
    override_dh_auto_install: [
      'install -Dm0644 mono/AdwaitaMono-Bold.ttf debian/fonts-adwaita-mono/usr/share/fonts/adwaita-mono/AdwaitaMono-Bold.ttf',
      'install -Dm0644 mono/AdwaitaMono-BoldItalic.ttf debian/fonts-adwaita-mono/usr/share/fonts/adwaita-mono/AdwaitaMono-BoldItalic.ttf',
      'install -Dm0644 mono/AdwaitaMono-Italic.ttf debian/fonts-adwaita-mono/usr/share/fonts/adwaita-mono/AdwaitaMono-Italic.ttf',
      'install -Dm0644 mono/AdwaitaMono-Regular.ttf debian/fonts-adwaita-mono/usr/share/fonts/adwaita-mono/AdwaitaMono-Regular.ttf',
    ],
  },
)
