use crate::Config;
use anyhow::{Context as _, Result, bail};
use std::path::PathBuf;

pub(crate) fn parse_input(path: PathBuf) -> Result<Vec<Config>> {
    let ext = path
        .extension()
        .context("failed to get extension of the input file")?
        .to_str()
        .context("non-utf8 input file extension")?;

    match ext {
        "list" => parse_list(path),
        "toml" => parse_toml(path),
        _ => bail!("input file ({path:?}) is neither .list nor .toml"),
    }
}

fn parse_list(path: PathBuf) -> Result<Vec<Config>> {
    let content =
        std::fs::read_to_string(&path).with_context(|| format!("failed to open {:?}", path))?;

    let mut configs = vec![];
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let path = path
            .parent()
            .with_context(|| format!("failed to get parent of {path:?}"))?
            .join(line);
        let config = Config::new(path)?;
        configs.push(config);
    }

    Ok(configs)
}

fn parse_toml(path: PathBuf) -> Result<Vec<Config>> {
    Ok(vec![Config::new(path)?])
}
