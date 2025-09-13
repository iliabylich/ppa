mod changelog;
mod control;
mod rules;
mod source;
mod version;

use crate::{
    macros::error,
    toml::{TomlPath, TomlTableWithPath, TomlValueWithPath},
};
use boml::table::TomlTable;
pub(crate) use changelog::Changelog;
pub(crate) use control::Control;
pub(crate) use rules::Rules;
pub(crate) use source::{GitClone, Source};
use std::{collections::HashMap, path::PathBuf};
pub use version::Version;

#[derive(Debug)]
pub struct Config {
    pub package_name: String,
    pub filepath: PathBuf,

    pub version: Version,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source: Source,
    pub(crate) debian: Debian,
    pub(crate) arch: String,
    pub(crate) binstall: Vec<String>,

    pub(crate) env: HashMap<String, String>,
    pub(crate) path: Vec<String>,
    pub(crate) additionally_produced_packages: Vec<String>,
}

impl Config {
    fn from_toml(toml: &TomlTableWithPath, package_name: String, filepath: PathBuf) -> Self {
        let version = Version::from_toml(toml.enter("version"));
        let dependencies = toml.enter("dependencies").into_array_of_strings();
        let source = Source::from_toml(toml.enter("source"));
        let debian = Debian::from_toml(toml.enter("debian"));
        let arch = toml.enter("arch").into_string();
        let binstall = toml
            .try_enter("binstall")
            .map(|a| a.into_array_of_strings())
            .unwrap_or_default();
        let env = toml
            .try_enter("env")
            .map(|h| h.into_table().into_string_string_map())
            .unwrap_or_default();
        let path = toml
            .try_enter("path")
            .map(|a| a.into_array_of_strings())
            .unwrap_or_default();
        let additionally_produced_packages = toml
            .try_enter("additionally_produced_packages")
            .map(|a| a.into_array_of_strings())
            .unwrap_or_default();

        Self {
            package_name,
            filepath,
            version,
            dependencies,
            source,
            debian,
            arch,
            binstall,
            env,
            path,
            additionally_produced_packages,
        }
    }

    pub fn read(path: PathBuf) -> Self {
        let package_name = path
            .with_extension("")
            .file_name()
            .unwrap_or_else(|| error!("failed to get base filename from {path:?}"))
            .to_str()
            .unwrap_or_else(|| error!("not a UTF-8 path"))
            .to_string();

        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| error!(err = err, "failed to read {path:?}"));

        let toml = boml::parse(&content)
            .unwrap_or_else(|err| error!(err = err, "failed to parse {path:?}"));

        let table = TomlTable::from(toml);
        let toml = TomlTableWithPath::new(&table, TomlPath::empty());

        Self::from_toml(&toml, package_name, path)
    }

    pub fn git_user_and_repo(&self) -> Option<(String, String)> {
        let Source::GitClone(GitClone { url, .. }) = &self.source else {
            return None;
        };

        let (user, repo) = url
            .strip_prefix("https://github.com/")?
            .strip_suffix(".git")?
            .split_once('/')?;
        Some((user.to_string(), repo.to_string()))
    }

    pub fn git_branch_or_tag(&self) -> Option<String> {
        let Source::GitClone(GitClone { branch_or_tag, .. }) = &self.source else {
            return None;
        };

        if branch_or_tag.bytes().all(|byte| !byte.is_ascii_digit()) {
            return None;
        }

        Some(branch_or_tag.clone())
    }

    pub fn relative_file_path(&self) -> String {
        let dir = std::env::current_dir()
            .unwrap_or_else(|err| error!("failed to get working directory: {err:?}"));
        let path = self
            .filepath
            .strip_prefix(dir)
            .unwrap_or_else(|err| error!("failed to string directory prefix: {err:?}"));
        path.to_str()
            .unwrap_or_else(|| error!("non-utf8 filepath {path:?}"))
            .to_string()
    }
}

#[derive(Debug)]
pub(crate) struct Debian {
    pub(crate) control: Control,
    pub(crate) rules: Option<Rules>,
}

impl Debian {
    fn from_toml(toml: TomlValueWithPath) -> Self {
        let table = toml.into_table();

        let control = Control::from_toml(table.enter("control"));
        let rules = table.try_enter("rules").map(Rules::from_toml);

        Self { control, rules }
    }
}
