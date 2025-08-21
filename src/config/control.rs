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

    pub(crate) fn render(&self, package_name: &str, arch: &str) -> String {
        format!(
            "Source: {package_name}
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: {package_name}
Section: utils
Priority: extra
Architecture: {arch}
Depends: {dependencies}
Description: {description}
",
            dependencies = self.dependencies.join(", "),
            description = self.description,
        )
    }
}
