use super::{TomlPath, TomlValueWithPath};
use boml::table::TomlTable;
use std::collections::HashMap;

pub(crate) struct TomlTableWithPath<'a> {
    table: &'a TomlTable<'a>,
    path: TomlPath,
}

impl<'a> TomlTableWithPath<'a> {
    pub(crate) fn new(table: &'a TomlTable<'a>, path: impl Into<TomlPath>) -> Self {
        Self {
            table,
            path: path.into(),
        }
    }

    pub(crate) fn try_enter(&self, key: &str) -> Option<TomlValueWithPath<'a>> {
        let value = self.table.get(key)?;
        Some(TomlValueWithPath::new(value, self.path.join(key)))
    }

    pub(crate) fn enter(&self, key: &str) -> TomlValueWithPath<'a> {
        self.try_enter(key)
            .unwrap_or_else(|| error!("path {}.{key} does not exist", self.path))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (String, TomlValueWithPath<'a>)> {
        self.table.iter().map(|(key, value)| {
            let key = key.to_string();
            let value = TomlValueWithPath::new(value, self.path.join(&key));
            (key, value)
        })
    }

    pub(crate) fn into_string_string_map(self) -> HashMap<String, String> {
        self.iter().map(|(k, v)| (k, v.into_string())).collect()
    }
}
