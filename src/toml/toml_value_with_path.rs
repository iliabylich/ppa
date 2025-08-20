use super::{TomlPath, TomlTableWithPath};
use boml::types::TomlValue;

pub(crate) struct TomlValueWithPath<'a> {
    value: &'a TomlValue<'a>,
    path: TomlPath,
}

impl<'a> TomlValueWithPath<'a> {
    pub(crate) fn new(value: &'a TomlValue<'a>, path: impl Into<TomlPath>) -> Self {
        Self {
            value,
            path: path.into(),
        }
    }

    pub(crate) fn try_to_string(&self) -> Option<String> {
        Some(self.value.as_string()?.to_string())
    }

    pub(crate) fn into_string(self) -> String {
        if let Some(string) = self.try_to_string() {
            string
        } else {
            error!("{} must be a string", self.path)
        }
    }

    pub(crate) fn try_to_table(&self) -> Option<TomlTableWithPath<'a>> {
        let table = self.value.as_table()?;
        Some(TomlTableWithPath::new(table, self.path.clone()))
    }

    pub(crate) fn into_table(self) -> TomlTableWithPath<'a> {
        self.try_to_table()
            .unwrap_or_else(|| error!("{} must be a table", self.path))
    }

    pub(crate) fn into_array(self) -> Vec<TomlValueWithPath<'a>> {
        let array = match self.value {
            TomlValue::Array(array, _) => array,
            _ => error!("{} must be an array", self.path),
        };

        array
            .iter()
            .enumerate()
            .map(|(idx, value)| TomlValueWithPath::new(value, self.path.join(format!("{idx}"))))
            .collect()
    }

    pub(crate) fn into_array_of_strings(self) -> Vec<String> {
        self.into_array()
            .into_iter()
            .map(|item| item.into_string())
            .collect()
    }
}
