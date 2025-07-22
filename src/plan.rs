use crate::{
    action::{Action, Exec},
    colors::{GREEN, RESET, YELLOW},
};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Plan {
    actions: Vec<Action>,
    env: HashMap<String, String>,
    path: Vec<String>,
}

impl Plan {
    pub(crate) fn new(env: HashMap<String, String>, path: Vec<String>) -> Self {
        Self {
            actions: vec![],
            env,
            path,
        }
    }

    pub(crate) fn add_env(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env.insert(key.into(), value.into());
    }

    pub(crate) fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub(crate) fn explain(self) {
        if !self.env.is_empty() {
            println!("{GREEN}ENV:{RESET}");
            for (key, val) in self.env {
                println!("{YELLOW}{key}={val}{RESET}");
            }
            println!();
        }

        if !self.path.is_empty() {
            println!("{GREEN}PATH (additional):{RESET}");
            for path in self.path {
                println!("{YELLOW}{path}{RESET}");
            }
            println!();
        }

        for action in self.actions {
            println!("{}\n", action.explanation());
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        for script in self.actions {
            script.exec(&self.env, &self.path)?;
        }
        Ok(())
    }
}
