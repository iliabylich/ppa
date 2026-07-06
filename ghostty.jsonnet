local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='ghostty',
  description='Ghostty is a fast, feature-rich, and cross-platform terminal emulator that uses platform-native UI and GPU acceleration.',
  version='1.2.3.tip20251224',
  path=['/root/zig-x86_64-linux-0.15.2'],
  install=[
    'libgtk-4-dev',
    'libadwaita-1-dev',
    'git',
    'blueprint-compiler',
    'libgtk4-layer-shell-dev',
  ],
  source={
    type: 'git',
    url: 'https://github.com/ghostty-org/ghostty.git',
    rev: 'tip',
  },
  scripts=[
    'wget -q https://ziglang.org/download/0.15.2/zig-x86_64-linux-0.15.2.tar.xz -O /root/zig-x86_64-linux-0.15.2.tar.xz',
    'tar xf /root/zig-x86_64-linux-0.15.2.tar.xz -C /root',
    'ls -l /root/zig-x86_64-linux-0.15.2',
  ],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_configure: ['@true'],
    override_dh_auto_build: ['@true'],
    override_dh_auto_install: [
      'zig version',
      'DESTDIR=$$PWD/debian/ghostty zig build --prefix /usr -Doptimize=ReleaseFast -fsys=fontconfig',
      'rm -f debian/ghostty/usr/share/terminfo/g/ghostty',
    ],
  },
)
