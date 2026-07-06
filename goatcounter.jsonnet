local configure = import 'scripts/base.jsonnet';

configure(
  file=std.thisFile,
  name='goatcounter',
  description='Easy web analytics. No tracking of personal data.',
  arch='amd64',
  version='2.7.0',
  install=[],
  source={
    type: 'git',
    url: 'https://github.com/arp242/goatcounter.git',
    rev: 'v2.7.0',
  },
  scripts=[],
  dependencies=['${shlibs:Depends}'],
  rules={
    '%': ['dh $@'],
    override_dh_auto_build: [
      'wget https://github.com/arp242/goatcounter/releases/download/v2.7.0/goatcounter-v2.7.0-linux-amd64.gz',
      'gunzip goatcounter-v2.7.0-linux-amd64.gz',
    ],
    override_dh_auto_configure: ['@true'],
    override_dh_auto_install: [
      'install -Dm0755 goatcounter-v2.7.0-linux-amd64 debian/goatcounter/usr/bin/goatcounter',
      'install -Dm0644 /shared/files/goatcounter.service debian/goatcounter/usr/lib/systemd/system/goatcounter.service',
    ],
  },
)
