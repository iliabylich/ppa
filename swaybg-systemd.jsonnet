local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='swaybg-systemd',
  description='Systemd service file for swaybg',
  version='1.0.1',
  source={ type: 'none' },
  dependencies=['swaybg'],
  rules={
    override_dh_auto_configure: ['@true'],
    override_dh_auto_build: ['@true'],
    override_dh_auto_install: [
      'install -Dm0644 /shared/files/swaybg-systemd.service debian/swaybg-systemd/usr/lib/systemd/user/swaybg.service',
    ],
  },
)
