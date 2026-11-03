local configure = import 'scripts/base.jsonnet';
local version = '2.0.0';

configure(
  file=std.thisFile,
  name='make-btrfs-snapshot',
  description='a set of tools to automatically create btrfs snapshots and update grub on any dpkg operation',
  version=version,
  install=['make'],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/make-btrfs-snapshot.git',
    rev: 'v' + version,
  },
  dependencies=[
    'grub2-common',
    'grub-common',
    'btrfs-progs',
  ],
  rules={
    override_dh_auto_build: ['make build'],
    override_dh_auto_install: ['DESTDIR="debian/make-btrfs-snapshot" make install'],
  },
)
