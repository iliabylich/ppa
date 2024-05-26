require 'open-uri'
require 'json'

REPOS = [
  'iliabylich/hn-parser',
  'iliabylich/commentary',
  'iliabylich/obj-load',
  'iliabylich/hyprwayland-scanner-deb',
  'iliabylich/hyprcursor-deb',
  'iliabylich/hyprland-deb',
]

`rm -rf *.deb`

def download_json(url)
  JSON.parse(URI.open(url).read)
end

def get_latest_release(repo)
  url = "https://api.github.com/repos/#{repo}/releases/latest"
  download_json(url)
end

def get_assets(repo, release_id)
  url = "https://api.github.com/repos/#{repo}/releases/#{release_id}/assets"
  download_json(url)
end

REPOS.each do |repo|
  releases = download_json("https://api.github.com/repos/#{repo}/releases")
  latest = releases.detect { |release| release['name'] == 'latest' }
  raise "Failed to find a release called 'latest' in #{repo}" if latest.nil?

  assets = latest['assets']
  deb_asset = assets
    .select { |asset| asset['name'].end_with?('.deb') }
    .max_by { |asset| asset['created_at'] }

  name = deb_asset['name']
  download_url = deb_asset['browser_download_url']

  puts "Downloading #{name}..."
  `wget #{download_url} -O #{name}`
end
