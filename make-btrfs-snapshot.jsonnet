local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='make-btrfs-snapshot',
  description='a set of tools to automatically create btrfs snapshots and update grub on any dpkg operation',
  arch='amd64',
  version='2.0.0',
  install=['make'],
  source={
    type: 'git',
    url: 'https://github.com/iliabylich/make-btrfs-snapshot.git',
    rev: 'v2.0.0',
  },
  scripts=[],
  dependencies=[
    'grub2-common',
    'grub-common',
    'btrfs-progs',
  ],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: ['make build'],
    override_dh_auto_install: ['DESTDIR="debian/make-btrfs-snapshot" make install'],
  },
)
