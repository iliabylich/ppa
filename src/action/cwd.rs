use crate::{
    action::Exec,
    colors::{GREEN, NC},
};
use anyhow::{Context as _, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Cwd {
    dir: String,
}

impl Cwd {
    pub(crate) fn new(dir: String) -> Self {
        Self { dir }
    }
}

impl Exec for Cwd {
    fn exec(&self, _env: &HashMap<String, String>, _path: &[String]) -> Result<()> {
        std::env::set_current_dir(&self.dir)
            .with_context(|| format!("failed to change working directory to {}", self.dir))
    }

    fn explanation(&self) -> String {
        format!("{GREEN}cwd {}{NC}", self.dir)
    }
}
