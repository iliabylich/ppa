local direct = import 'scripts/direct.jsonnet';

local version = '1.0.7';

direct(
  file=std.thisFile,
  name='mpclipboard-client',
  version=version,
  url='https://github.com/iliabylich/mpclipboard/releases/download/latest/mpclipboard-client_' + version + '.deb',
)
