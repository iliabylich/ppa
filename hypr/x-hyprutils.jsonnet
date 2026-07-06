local configure = import '../scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='x-hyprutils',
  description='Hyprland utilities library used across the ecosystem',
  version='0.12.0',
  install=['libpixman-1-dev'],
  source={
    type: 'git',
    url: 'https://github.com/hyprwm/hyprutils.git',
    rev: 'v0.12.0',
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_configure: [
      'cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:PATH=/usr -S . -B ./build',
    ],
    override_dh_auto_build: [
      'cmake --build ./build --config Release --target all -j`nproc`',
    ],
    override_dh_auto_install: [
      'cmake --install build --prefix debian/x-hyprutils/usr',
    ],
  },
)
