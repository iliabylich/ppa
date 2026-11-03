local configure = import 'scripts/base.jsonnet';
local version = '2.7.0';

configure(
  file=std.thisFile,
  name='goatcounter',
  description='Easy web analytics. No tracking of personal data.',
  version=version,
  source={
    type: 'release',
    url: 'https://github.com/arp242/goatcounter.git',
    rev: 'v' + version,
  },
  dependencies=['${shlibs:Depends}'],
  rules={
    override_dh_auto_build: [
      'wget https://github.com/arp242/goatcounter/releases/download/v' + version + '/goatcounter-v' + version + '-linux-amd64.gz',
      'gunzip goatcounter-v' + version + '-linux-amd64.gz',
    ],
    override_dh_auto_configure: ['@true'],
    override_dh_auto_install: [
      'install -Dm0755 goatcounter-v' + version + '-linux-amd64 debian/goatcounter/usr/bin/goatcounter',
      'install -Dm0644 /shared/files/goatcounter.service debian/goatcounter/usr/lib/systemd/system/goatcounter.service',
    ],
  },
)
