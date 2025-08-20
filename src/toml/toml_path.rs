#[derive(Clone)]
pub(crate) struct TomlPath(Vec<String>);

impl From<String> for TomlPath {
    fn from(component: String) -> Self {
        Self(vec![component])
    }
}

impl TomlPath {
    pub(crate) fn empty() -> Self {
        Self(vec![])
    }

    pub(crate) fn join(&self, component: impl Into<String>) -> Self {
        let mut new_path = self.0.clone();
        new_path.push(component.into());
        Self(new_path)
    }
}

impl std::fmt::Display for TomlPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("."))
    }
}
