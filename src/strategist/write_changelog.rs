use crate::{action::Action, plan::Plan, templates::Templates};
use anyhow::Result;

pub(crate) fn write_changelog(
    plan: &mut Plan,
    build_dir: &str,
    package_name: &str,
    version: &str,
    templates: &Templates,
) -> Result<()> {
    plan.push(
        Action::cmd()
            .exe("mkdir")
            .arg("-p")
            .arg(format!("{build_dir}/debian"))
            .finish(),
    );

    let contents = templates.changelog(package_name, version)?;
    plan.push(
        Action::write()
            .path(format!("{build_dir}/debian/changelog"))
            .contents(contents),
    );

    Ok(())
}
