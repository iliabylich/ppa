use anyhow::{Context as _, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) package_name: String,
    #[serde(skip)]
    pub(crate) filepath: PathBuf,

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
    #[serde(default)]
    pub(crate) additionally_produced_packages: Vec<String>,
}

impl Config {
    pub(crate) fn new(path: PathBuf) -> Result<Self> {
        let package_name = path
            .with_extension("")
            .file_name()
            .with_context(|| format!("failed to get base filename from {path:?}"))?
            .to_str()
            .context("not a UTF-8 path")?
            .to_string();

        let content =
            std::fs::read_to_string(&path).with_context(|| format!("failed to read {path:?}"))?;

        let mut config: Config =
            toml::from_str(&content).with_context(|| format!("failed to parse {path:?}"))?;

        config.filepath = path;
        config.package_name = package_name;

        Ok(config)
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
