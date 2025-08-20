use crate::{action::Action, green, yellow};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Plan {
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

    pub fn explain(self) {
        if !self.env.is_empty() {
            green!("ENV:");
            for (key, val) in self.env {
                yellow!("{key}={val}");
            }
            eprintln!();
        }

        if !self.path.is_empty() {
            green!("PATH (additional):");
            for path in self.path {
                yellow!("{path}");
            }
            eprintln!();
        }

        for action in self.actions {
            action.explain();
        }
    }

    pub fn run(self) {
        for action in self.actions {
            action.explain();
            action.exec(&self.env, &self.path);
        }
    }
}
