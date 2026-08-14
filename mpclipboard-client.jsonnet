local direct = import 'scripts/direct.jsonnet';
local version = '2.0.3';

direct(
  file=std.thisFile,
  name='mpclipboard-client',
  version=version,
  url='https://github.com/iliabylich/mpclipboard/releases/download/latest/mpclipboard-client_' + version + '.deb',
)
