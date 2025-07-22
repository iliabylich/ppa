use crate::{action::Action, plan::Plan, templates::Templates};
use anyhow::Result;
use std::collections::HashMap;

pub(crate) fn write_rules(
    plan: &mut Plan,
    build_dir: &str,
    targets: HashMap<String, Vec<String>>,
    templates: &Templates,
) -> Result<()> {
    plan.push(
        Action::cmd()
            .exe("mkdir")
            .arg("-p")
            .arg(format!("{build_dir}/debian"))
            .finish(),
    );

    let contents = templates.rules(targets)?;
    plan.push(
        Action::write()
            .path(format!("{build_dir}/debian/rules"))
            .contents(contents),
    );
    plan.push(
        Action::cmd()
            .exe("chmod")
            .arg("+x")
            .arg(format!("{build_dir}/debian/rules"))
            .finish(),
    );

    Ok(())
}
