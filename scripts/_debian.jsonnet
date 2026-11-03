{
  changelog(name, version)::
    std.join('\n', [
      name + ' (' + version + ') unstable; urgency=low',
      '',
      '  * Release',
      '',
      ' -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000',
    ]),

  control(name, description, arch, dependencies)::
    std.join('\n', std.prune([
      'Source: ' + name,
      'Section: utils',
      'Priority: optional',
      'Maintainer: John Doe <john@doe.org>',
      'Build-Depends: debhelper-compat (= 14)',
      'Standards-Version: 4.7.4',
      '',
      'Package: ' + name,
      'Section: utils',
      'Priority: optional',
      'Architecture: ' + arch,
      if std.length(dependencies) > 0 then 'Depends: ' + std.join(', ', dependencies),
      'Description: ' + std.trim(description),
    ])),

  rules(rules)::
    local default = {
      '%': ['dh $@ --without=single-binary'],
      override_dh_auto_test: ['@true'],
      override_dh_strip: ['@true'],
    };
    local prefix = [
      '#!/usr/bin/make -f',
      'export DH_VERBOSE = 1',
      '',
    ];

    local renderOne = function(target, statements)
      target + ':\n' + std.join('\n', std.map(function(s) '\t' + s, statements));

    local renderList = function(list)
      std.map(
        function(kv) renderOne(kv.key, kv.value),
        list
      );

    local asSortedArray = function(rulesAsObject)
      std.sort(
        std.objectKeysValues(rulesAsObject),
        function(kv) if kv.key == '%' then 0 else 1
      );

    local lines = std.flattenDeepArray([
      prefix,
      renderList(
        asSortedArray(default + rules)
      ),
    ]);
    std.join('\n', lines),
}
