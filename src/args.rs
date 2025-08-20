use crate::{error, list::List};
use std::path::PathBuf;

fn dir_from_env() -> PathBuf {
    if std::env::var("DOCKER").is_ok() {
        PathBuf::from("/shared")
    } else {
        std::env::current_dir()
            .unwrap_or_else(|err| error!(err = err, "failed to get working directory"))
    }
}

pub fn paths_from_args() -> Vec<PathBuf> {
    let dir = dir_from_env();
    let mut paths = vec![];

    for filename in std::env::args().skip(1) {
        let path = dir.join(filename);

        if path.ends_with("Cargo.toml") || path.ends_with("config.toml") {
            continue;
        }

        let ext = path
            .extension()
            .unwrap_or_else(|| error!("failed to determine extension of {path:?}"))
            .to_str()
            .unwrap_or_else(|| error!("non-utf8 extension of {path:?}"))
            .to_string();

        match ext.as_str() {
            "toml" => paths.push(path),
            "list" => {
                println!("reading list");
                for inner in List::read(path) {
                    paths.push(inner);
                }
            }
            _ => error!("unsupported input file extension {ext}"),
        }
    }

    if paths.is_empty() {
        error!("No paths given, exiting");
    }

    paths
}
