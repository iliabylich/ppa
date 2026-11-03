local configure = import 'scripts/base.jsonnet';
local version = '1.4.0';
local zigVersion = '0.17.0';

configure(
  file=std.thisFile,
  name='ghostty',
  description='Ghostty is a fast, feature-rich, and cross-platform terminal emulator that uses platform-native UI and GPU acceleration.',
  version=version,
  path=['/root/zig-x86_64-linux-' + zigVersion],
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
    rev: 'v' + version,
  },
  scripts=[
    'wget -q https://ziglang.org/download/' + zigVersion + '/zig-x86_64-linux-' + zigVersion + '.tar.xz -O /root/zig-x86_64-linux-' + zigVersion + '.tar.xz',
    'tar xf /root/zig-x86_64-linux-' + zigVersion + '.tar.xz -C /root',
    'ls -l /root/zig-x86_64-linux-' + zigVersion,
    'zig version',
  ],
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_configure: ['@true'],
    override_dh_auto_build: ['@true'],
    override_dh_auto_install: [
      'DESTDIR=$$PWD/debian/ghostty zig build --prefix /usr -Doptimize=ReleaseFast -fsys=fontconfig',
    ],
  },
)
