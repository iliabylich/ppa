local download = function(name, version, url)
  local debfile = name + '_' + version + '_amd64.deb';

  std.join('\n', std.flattenDeepArray([
    '#!/usr/bin/env bash',
    '',
    'set -euo pipefail',
    "export PS4=$'\\e[1;35m>>> \\e[0m'",
    'set -x',
    '',
    'mkdir -p /build',
    'cd /build',
    'wget ' + url + ' -O ' + debfile,
    '',
    'cp /build/' + debfile + ' /shared/' + debfile,
    'mkdir -p /shared/deb-latest',
    'cp /build/' + debfile + ' /shared/deb-latest/' + name + '.deb',
  ]));

function(file, name, version, arch='amd64', url)
  function(action)
    if action == 'dump' then
      error 'unsupported'
    else if action == 'render' then
      download(name, version, url)
    else if action == 'check-updates' then
      'echo -e "' + file + ': \\e[0;33mskipping (direct)\\e[0m"'
    else
      error 'unknown action ' + action
