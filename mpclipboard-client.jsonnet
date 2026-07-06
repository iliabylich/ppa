local direct = import 'scripts/direct.jsonnet';

direct(
  file=std.thisFile,
  name='mpclipboard-client',
  version='1.0.6',
  url='https://github.com/iliabylich/mpclipboard/releases/download/latest/mpclipboard-client_1.0.6.deb',
)
