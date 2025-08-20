use crate::{
    action::Exec,
    colors::{NC, YELLOW},
};
use anyhow::{Context as _, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Write {
    path: String,
    contents: String,
}

impl Write {
    pub(crate) fn new(path: String, contents: String) -> Self {
        Self { path, contents }
    }
}

impl Exec for Write {
    fn exec(&self, _env: &HashMap<String, String>, _path: &[String]) -> Result<()> {
        std::fs::write(&self.path, &self.contents)
            .with_context(|| format!("Failed to write to {}", self.path))
    }

    fn explanation(&self) -> String {
        format!("{YELLOW}Writing to {}:{NC}\n{}", self.path, self.contents)
    }
}
