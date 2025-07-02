use anyhow::{Context as _, Result};
use std::collections::HashMap;

pub(crate) struct Templates {
    engine: upon::Engine<'static>,
}

impl Templates {
    pub(crate) fn new() -> Result<Self> {
        let mut engine = upon::Engine::new();

        engine
            .add_template("changelog", include_str!("./changelog"))
            .context("failed to add changelog template")?;
        engine
            .add_template("compat", include_str!("./compat"))
            .context("failed to add compat template")?;
        engine
            .add_template("control", include_str!("./control"))
            .context("failed to add control template")?;
        engine
            .add_template("rules", include_str!("./rules"))
            .context("failed to add rules template")?;

        Ok(Self { engine })
    }

    pub(crate) fn changelog(&self, package_name: &str, version: &str) -> Result<String> {
        self.engine
            .template("changelog")
            .render(upon::value! {
                package_name: package_name,
                version: version
            })
            .to_string()
            .context("failed to render changelog template")
    }

    pub(crate) fn compat(&self, compat: u8) -> Result<String> {
        self.engine
            .template("compat")
            .render(upon::value! { compat: compat })
            .to_string()
            .context("failed to render compat template")
    }

    pub(crate) fn control(
        &self,
        package_name: &str,
        arch: &str,
        dependencies: &[String],
        description: &str,
    ) -> Result<String> {
        self.engine
            .template("control")
            .render(upon::value! {
                package_name: package_name,
                arch: arch,
                dependencies: dependencies.join(", "),
                description: description
            })
            .to_string()
            .context("failed to render control template")
    }

    pub(crate) fn rules(&self, mut targets: HashMap<String, Vec<String>>) -> Result<String> {
        let mut out = vec![];
        if let Some((target, lines)) = targets.remove_entry("%") {
            out.push(format!("{target}:"));
            for line in lines {
                out.push(format!("\t{line}"));
            }
        }

        out.push("override_dh_auto_test:".to_string());
        out.push("\techo \"skip\"".to_string());

        for (target, lines) in targets {
            out.push(format!("{target}:"));
            for line in lines {
                out.push(format!("\t{line}"));
            }
        }
        let targets = out.join("\n");
        self.engine
            .template("rules")
            .render(upon::value! { targets: targets })
            .to_string()
            .context("failed to render rules templates")
    }
}
