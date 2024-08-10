#!/usr/bin/env ruby

require 'open-uri'
require 'json'

REPO = ARGV[0]
RELEASE_NAME = ARGV[1]

all_releases_url = "https://api.github.com/repos/#{REPO}/releases"

releases = JSON.parse(URI.open(all_releases_url).read)

release = releases.find { |r| r['name'] == RELEASE_NAME }

release['assets'].each do |asset|
  `wget "#{asset['browser_download_url']}" -O "#{asset['name']}"`
end
