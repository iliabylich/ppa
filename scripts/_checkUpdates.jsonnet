local ownerRepoTagParser = {
  PREFIX: 'https://github.com/',
  SUFFIX: '.git',

  // (String, String) -> String
  trimPrefix(str, prefix)::
    std.splitLimit(str, prefix, 1)[1],

  // (String, String) -> String
  trimSuffix(str, suffix)::
    std.splitLimitR(str, suffix, 1)[0],

  // (String) -> (String, null) | (null, err)
  extractOwnerSlashRepo(url)::
    if std.startsWith(url, self.PREFIX) && std.endsWith(url, self.SUFFIX) then
      local ownerSlashRepo = self.trimSuffix(self.trimPrefix(url, self.PREFIX), self.SUFFIX);
      [ownerSlashRepo, null]
    else
      [null, 'git URL must have a "https://github.com/" prefix and a ".git" suffix ' + url],


  // (String) -> (String, null) | (err, null)
  extractRev(rev)::
    local chars = std.stringChars(rev);
    local digits = std.map(function(n) std.toString(n), std.range(0, 9));
    local isDigit = function(c) std.contains(digits, c);
    local hasAtLeastOneDigit = std.any(std.map(isDigit, chars));
    if hasAtLeastOneDigit then
      [rev, null]
    else
      [null, 'rev does not look like a tag: ' + rev],

  // ({ type: String, url: String, rev: String }) -> ((String, String), null) | ((null, null), err)
  parse(source)::
    if source.type != 'git' then
      [[null, null], 'not a git source']
    else
      local ownerSlashRepoOrErr = self.extractOwnerSlashRepo(source.url);
      local ownerSlashRepo = ownerSlashRepoOrErr[0];
      local err = ownerSlashRepoOrErr[1];
      if err != null then
        [[null, null], err]
      else
        local revOrErr = self.extractRev(source.rev);
        local rev = revOrErr[0];
        local err = revOrErr[1];
        if err != null then
          [[null, null], err]
        else
          [[ownerSlashRepo, rev], null],
};


local render = function(file, dataOrErr)
  local data = dataOrErr[0];
  local err = dataOrErr[1];
  local ownerSlashRepo = data[0];
  local rev = data[1];

  std.join('\n', std.flattenDeepArray([
    |||
      #!/usr/bin/env bash

      set -euo pipefail

      RED='\x1b[0;31m'
      GREEN='\x1b[0;32m'
      YELLOW='\x1b[0;33m'
      NC='\x1b[0m'
    |||,
    'FILE="' + file + '"',
    if err != null then
      [
        'ERR="' + err + '"',
        'echo -e "${FILE}: ${YELLOW}${ERR}${NC}"',
      ]
    else
      [
        'OWNER_SLASH_REPO="' + ownerSlashRepo + '"',
        'REV="' + rev + '"',
        |||
          LATEST=$(gh release view -R "$OWNER_SLASH_REPO" --json tagName --jq .tagName)

          if [ "$LATEST" = "$REV" ]; then
            echo -e "${FILE}: ${GREEN}current=${REV}, latest=${LATEST}${NC}"
          else
            echo -e "${FILE}: ${RED}current=${REV}, latest=${LATEST}${NC}"
          fi
        |||,
      ],
  ]));

function(config)
  render(config.file, ownerRepoTagParser.parse(config.source))
