local direct = import 'scripts/direct.jsonnet';

direct(
  file=std.thisFile,
  name='mpclipboard-server',
  version='1.0.3',
  url='https://github.com/iliabylich/mpclipboard/releases/download/latest/mpclipboard-server_1.0.3.deb',
)
