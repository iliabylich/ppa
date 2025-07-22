use crate::colors::{GREEN, RESET, YELLOW};
use anyhow::{Context as _, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};
use toml_edit::DocumentMut;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) package_name: String,
    #[serde(skip)]
    pub(crate) absolute_config_path: String,

    pub(crate) version: Version,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source: Source,
    pub(crate) debian: Debian,
    pub(crate) arch: String,
    #[serde(default)]
    pub(crate) binstall: Vec<String>,

    #[serde(default)]
    pub(crate) env: HashMap<String, String>,
    #[serde(default)]
    pub(crate) path: Vec<String>,
    pub(crate) additionally_produced_packages: Option<Vec<String>>,
}

impl Config {
    pub(crate) fn new(dir: &str, path: &str) -> Result<Self> {
        let absolute_config_path = format!("{dir}/{path}");

        let content = std::fs::read_to_string(&absolute_config_path)
            .with_context(|| format!("failed to read {absolute_config_path}"))?;

        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("failed to parse {absolute_config_path}"))?;

        config.absolute_config_path = absolute_config_path;

        config.package_name = Path::new(&config.absolute_config_path)
            .with_extension("")
            .file_name()
            .with_context(|| {
                format!(
                    "failed to get base filename from {}",
                    config.absolute_config_path
                )
            })?
            .to_str()
            .context("not a UTF-8 path")?
            .to_string();

        Ok(config)
    }

    pub(crate) fn bump_version_trailer(self) -> Result<()> {
        let path = self.absolute_config_path;

        let toml =
            std::fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;
        let mut doc = toml
            .parse::<DocumentMut>()
            .with_context(|| format!("failed to parse {path}"))?;

        let version = doc.get("version").context("no 'version' field")?;
        let Some(version) = version.get("specific") else {
            println!("[{path}] {YELLOW}Skipping, it has monotonically incrementing version{RESET}");
            return Ok(());
        };
        let version = version
            .as_str()
            .context("version is provided but it's not a string")?;

        let (base, trailer) = match version.split_once('-') {
            Some((base, trailer)) => {
                let trailer = trailer
                    .parse::<u32>()
                    .with_context(|| format!("non-numeric version trailer in {path}"))?;
                (base, Some(trailer))
            }
            None => (version, None),
        };
        let new_trailer = trailer.map(|v| v + 1).unwrap_or(1);
        let new_version = format!("{base}-{new_trailer}");

        println!("[{path}] {GREEN}Bumping {version} -> {new_version}{RESET}");

        doc["version"]["specific"] = toml_edit::value(new_version);

        std::fs::write(&path, doc.to_string())
            .with_context(|| format!("failed to update {path}"))?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Version {
    #[serde(rename = "0-0-stamp")]
    ZeroZeroTimestamp,

    #[serde(rename = "specific")]
    Specific(String),
}

#[derive(Deserialize, Debug)]
pub(crate) enum Source {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "git-clone")]
    GitClone {
        url: String,
        branch_or_tag: String,
        post_clone_scripts: Option<Vec<String>>,
    },
}

impl Source {
    pub(crate) fn git_url(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { url, .. } => Some(url),
        }
    }

    pub(crate) fn git_branch_or_tag(&self) -> Option<&str> {
        match self {
            Source::None => None,
            Source::GitClone { branch_or_tag, .. } => Some(branch_or_tag),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Debian {
    pub(crate) changelog: bool,
    pub(crate) control: Option<Control>,
    pub(crate) rules: Option<HashMap<String, Vec<String>>>,
    pub(crate) compat: Option<u8>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Control {
    pub(crate) dependencies: Vec<String>,
    pub(crate) description: String,
}
