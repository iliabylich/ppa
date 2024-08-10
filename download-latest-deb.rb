#!/usr/bin/env ruby

require 'open-uri'
require 'json'

repo = ARGV[0]

release_url = "https://api.github.com/repos/#{repo}/releases/latest"
puts "Download latest release metadata from #{release_url}"

release = JSON.parse(URI.open(release_url).read)
assets = release['assets']
debs = assets.select { |asset| asset['name'].end_with?('.deb') }
if debs.length > 1
  debs = debs.select { |asset| asset['name'].include?('x86') || asset['name'].include?('amd64') }
end

deb =
  if debs.length == 0
    raise "Unable to find .deb asset in the latest release of #{repo}"
  elsif debs.length == 1
    debs.first
  else
    raise "Too many debs, don't know which to choose in #{repo}"
  end

`wget "#{deb['browser_download_url']}" -O "#{deb['name']}"`
