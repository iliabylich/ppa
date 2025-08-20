use crate::error;
use std::path::PathBuf;

fn dir_from_env() -> PathBuf {
    if std::env::var("DOCKER").is_ok() {
        PathBuf::from("/shared")
    } else {
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("{err:?}");
            std::process::exit(1);
        })
    }
}

pub fn paths_from_args() -> Vec<PathBuf> {
    let dir = dir_from_env();

    let paths = std::env::args()
        .skip(1)
        .filter(|filename| !filename.ends_with("Cargo.toml"))
        .filter(|filename| !filename.ends_with("config.toml"))
        .map(|filename| dir.join(filename))
        .collect::<Vec<_>>();

    if paths.is_empty() {
        error!("No paths given, exiting");
    }

    paths
}
