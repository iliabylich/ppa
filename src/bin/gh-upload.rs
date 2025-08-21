use build_deb_package::{
    colors::{NC, YELLOW},
    error,
    github::{GitHub, GitHubAsset},
    green, red, yellow,
};
use std::{path::Path, time::Duration};

const USER: &str = "iliabylich";
const REPO: &str = "ppa";
const RELEASE: &str = "latest";

fn main() {
    let assets = std::env::args()
        .skip(1)
        .map(|filename| {
            validate(&filename);
            GitHubAsset::new(filename)
        })
        .collect::<Vec<_>>();

    let github = GitHub::new(USER, REPO);

    green!("Listing existing assets in {USER}/{REPO}/{RELEASE}",);
    let assets_on_github = github
        .list_assets(RELEASE)
        .unwrap_or_else(|err| error!("{err}"));

    let mut assets_to_remove = vec![];

    for asset in assets.iter() {
        if let Some(existing) = assets_on_github
            .iter()
            .find(|a| asset.package_name == a.package_name)
        {
            assets_to_remove.push(existing.clone())
        }
    }

    if !assets_to_remove.is_empty() {
        yellow!("About to removing assets:");
        for asset in assets_to_remove.iter() {
            yellow!("  - {asset}")
        }
    }

    eprint!("{YELLOW}Starting in ...3{NC}");
    std::thread::sleep(Duration::from_secs(1));
    eprint!("{YELLOW} ...2{NC}");
    std::thread::sleep(Duration::from_secs(1));
    eprint!("{YELLOW} ...1{NC}");
    std::thread::sleep(Duration::from_secs(1));
    eprintln!();

    for asset in assets_to_remove {
        github
            .delete_asset(RELEASE, &asset)
            .unwrap_or_else(|err| error!("failed to delete asset {asset}\n{err}"));
        red!("-- Asset {asset} has been removed");
    }

    for asset in assets {
        github
            .upload_asset(RELEASE, &asset)
            .unwrap_or_else(|err| error!("failed to upload asset {asset}\n{err}"));
        green!("++ Asset {asset} has been uploaded");
    }
}

fn validate(filename: &str) {
    let path = Path::new(filename);
    if path.components().collect::<Vec<_>>().len() != 1 {
        error!("{filename:?} is invalid: not a plain filename");
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("deb") {
        error!("{filename:?} is invalid: not a .deb file");
    }
    green!("{filename} is valid")
}
