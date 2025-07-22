use crate::{action::Action, plan::Plan, templates::Templates};
use anyhow::Result;

pub(crate) fn write_compat(
    plan: &mut Plan,
    build_dir: &str,
    compat: u8,
    templates: &Templates,
) -> Result<()> {
    plan.push(
        Action::cmd()
            .exe("mkdir")
            .arg("-p")
            .arg(format!("{build_dir}/debian"))
            .finish(),
    );

    let contents = templates.compat(compat)?;
    plan.push(
        Action::write()
            .path(format!("{build_dir}/debian/compat"))
            .contents(contents),
    );

    Ok(())
}
