local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='fonts-apple-color-emoji',
  description='Apple Color Emoji font',
  arch='all',
  version='20260219',
  install=['wget'],
  source={ type: 'none' },
  scripts=[],
  dependencies=[],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['wget -q https://github.com/samuelngs/apple-emoji-ttf/releases/download/macos-26-20260219-2aa12422/AppleColorEmoji-Linux.ttf -O AppleColorEmoji.ttf'],
    override_dh_auto_install: [
      'mkdir -p debian/fonts-apple-color-emoji/usr/share/fonts/apple-color-emoji',
      'install -m644 *.ttf debian/fonts-apple-color-emoji/usr/share/fonts/apple-color-emoji/',
    ],
  },
)
