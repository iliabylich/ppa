use crate::toml::TomlValueWithPath;

#[derive(Debug)]
pub(crate) struct Control {
    pub(crate) dependencies: Vec<String>,
    pub(crate) description: String,
}

impl Control {
    pub(crate) fn from_toml(toml: TomlValueWithPath) -> Self {
        let table = toml.into_table();

        let dependencies = table.enter("dependencies").into_array_of_strings();
        let description = table.enter("description").into_string();

        Self {
            dependencies,
            description,
        }
    }
}
