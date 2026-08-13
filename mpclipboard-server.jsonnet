local direct = import 'scripts/direct.jsonnet';
local version = '2.0.2';

direct(
  file=std.thisFile,
  name='mpclipboard-server',
  version=version,
  url='https://github.com/iliabylich/mpclipboard/releases/download/latest/mpclipboard-server_' + version + '.deb',
)
