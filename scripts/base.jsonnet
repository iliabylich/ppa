local checkUpdates = import './_checkUpdates.jsonnet';
local debian = import './_debian.jsonnet';
local render = import './_render.jsonnet';

function(
  file,
  name,
  description,
  arch='amd64',
  version,
  env={},
  path=[],
  install=[],
  binstall=[],
  source,
  scripts=[],
  dependencies,
  rules
)
  local config = {
    file: file,
    name: name,
    version: version,
    arch: arch,
    buildDir: '/build/' + name + '-' + version,

    env: env,
    path: path,

    install: install,
    binstall: binstall,

    source: source,
    scripts: scripts,

    debian: {
      changelog: debian.changelog(
        name=name,
        version=version
      ),
      compat: debian.compat(),
      control: debian.control(
        name=name,
        description=description,
        arch=arch,
        dependencies=dependencies
      ),
      rules: debian.rules(
        rules=rules
      ),
    },
  };

  function(action)
    if action == 'dump' then
      std.manifestJson(config)
    else if action == 'render' then
      render(config)
    else if action == 'check-updates' then
      checkUpdates(config)
    else
      error 'unknown action ' + action
