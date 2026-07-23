local exportEnvironmentVariables = function(env)
  local default = [
    { key: 'DEB_BUILD_OPTIONS', value: 'parallel=$(nproc)' },
    { key: 'CARGO_TERM_COLOR', value: 'always' },
  ];
  local sorted =
    std.sort(
      std.objectKeysValues(env),
      function(kv) kv.key
    );
  std.map(
    function(kv)
      'export ' + kv.key + '="' + kv.value + '"',
    default + sorted,
  );

local exportPath = function(path)
  if std.length(path) != 0 then
    'export PATH="' + std.join(':', path) + ':$PATH"'
  else
    '';

local aptInstall = function(install)
  if std.length(install) != 0 then
    [
      'apt update',
      'apt install --no-install-recommends --no-install-suggests -y ' + std.join(' ', install),
    ]
  else
    [];

local binstall = function(binstall)
  if std.length(binstall) != 0 then
    [
      'cargo binstall -y ' + std.join(' ', binstall),
    ] else
    [];

local fetchSource = function(source, buildDir)
  if source.type == 'git' then
    std.join(' ', [
      'git',
      'clone',
      source.url,
      '--filter=blob:none',
      '--recursive',
      '--shallow-submodules',
      '--depth=1',
      '-q',
      '--branch=' + source.rev,
      buildDir,
    ])
  else if source.type == 'none' then
    'mkdir -p ' + buildDir
  else if source.type == 'custom' then
    source.func(buildDir)
  else
    error 'Unknown source type "' + source.type + '"'
;


function(config)
  local debfile = config.name + '_' + config.version + '_' + config.arch + '.deb';

  std.join('\n', std.flattenDeepArray([
    '#!/usr/bin/env bash',
    '',
    'set -euo pipefail',
    "export PS4=$'\\e[1;35m>>> \\e[0m'",
    'set -x',

    exportEnvironmentVariables(config.env),
    exportPath(config.path),
    aptInstall(config.install),
    binstall(config.binstall),

    'mkdir -p /build',
    fetchSource(config.source, config.buildDir),
    'cd ' + config.buildDir,
    'ls -l --color=always',

    config.scripts,

    'mkdir -p debian',
    "tee debian/changelog <<'EOF'",
    config.debian.changelog,
    'EOF',
    "tee debian/compat <<'EOF'",
    config.debian.compat,
    'EOF',
    "tee debian/control <<'EOF'",
    config.debian.control,
    'EOF',
    "tee debian/rules <<'EOF'",
    config.debian.rules,
    'EOF',
    'chmod +x debian/rules',

    'dh binary',
    'ls -l /build --color=always',

    'cp /build/' + debfile + ' /shared/' + debfile,
    'mkdir -p /shared/deb-latest',
    'cp /build/' + debfile + ' /shared/deb-latest/' + config.name + '.deb',
  ]))
