use crate::{
    colors::{GREEN, NC, YELLOW},
    commands::CommandExec,
    config::{Config, Version},
};
use anyhow::{Context as _, Result};
use toml_edit::DocumentMut;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BumpVersionTrailer;

impl CommandExec for BumpVersionTrailer {
    fn exec(self, config: Config) -> Result<()> {
        let path = config.filepath;
        let package = config.package_name;

        let Version::Specific(version) = config.version else {
            println!(
                "[{package}] {YELLOW}Skipping, it has monotonically incrementing version{NC}"
            );
            return Ok(());
        };

        let Some(new_version) = bump_version(&version) else {
            eprintln!("non-numeric version trailer in {path:?}");
            return Ok(());
        };

        let toml =
            std::fs::read_to_string(&path).with_context(|| format!("failed to read {path:?}"))?;
        let mut doc = toml
            .parse::<DocumentMut>()
            .with_context(|| format!("failed to parse {path:?}"))?;

        println!("[{package}] {GREEN}Bumping {version} -> {new_version}{NC}");

        doc["version"]["specific"] = toml_edit::value(new_version);

        std::fs::write(&path, doc.to_string())
            .with_context(|| format!("failed to update {path:?}"))?;

        Ok(())
    }
}

fn bump_version(version: &str) -> Option<String> {
    let (base, trailer) = match version.split_once('-') {
        Some((base, trailer)) => {
            let Ok(trailer) = trailer.parse::<u32>() else {
                return None;
            };
            (base, Some(trailer))
        }
        None => (version, None),
    };
    let new_trailer = trailer.map(|v| v + 1).unwrap_or(1);
    let new_version = format!("{base}-{new_trailer}");
    Some(new_version)
}
