use crate::config::Version;
use anyhow::{Context as _, Result};

pub(crate) fn resolve_version(version: Version) -> Result<String> {
    let version = match version {
        Version::ZeroZeroTimestamp => format!("0.0.{}", date()?),
        Version::Specific(version) => version,
    };
    Ok(version)
}

fn date() -> Result<String> {
    let stdout = std::process::Command::new("date")
        .arg("+%s")
        .output()
        .context("failed to get timestamp")?
        .stdout;

    let out = String::from_utf8(stdout)
        .context("non-utf-8 output of ")?
        .trim()
        .to_string();
    Ok(out)
}
