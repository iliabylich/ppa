use crate::{error, toml::TomlValueWithPath};

#[derive(Debug)]
pub enum Version {
    ZeroZeroTimestamp,
    Specific(String),
}
impl Version {
    pub(crate) fn from_toml(toml: TomlValueWithPath) -> Self {
        let version = toml.into_string();

        match version.as_str() {
            "0-0-stamp" => Self::ZeroZeroTimestamp,
            specific => Self::Specific(specific.to_string()),
        }
    }

    pub(crate) fn resolve(&self) -> String {
        match self {
            Version::ZeroZeroTimestamp => format!("0.0.{}", date()),
            Version::Specific(version) => version.clone(),
        }
    }
}

fn date() -> String {
    let stdout = std::process::Command::new("date")
        .arg("+%s")
        .output()
        .unwrap_or_else(|err| error!(err = err, "failed to get timestamp"))
        .stdout;

    String::from_utf8(stdout)
        .unwrap_or_else(|err| error!(err = err, "non-utf-8 output of "))
        .trim()
        .to_string()
}
