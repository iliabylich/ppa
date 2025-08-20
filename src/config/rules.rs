use crate::toml::TomlValueWithPath;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Rules {
    pub(crate) map: Option<HashMap<String, Vec<String>>>,
}

impl Rules {
    pub(crate) fn from_toml(toml: Option<TomlValueWithPath>) -> Self {
        let map = toml.map(|rules| {
            rules
                .into_table()
                .iter()
                .map(|(k, v)| (k, v.into_array_of_strings()))
                .collect::<HashMap<_, _>>()
        });
        Self { map }
    }
}
