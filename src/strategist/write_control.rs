use crate::{action::Action, config::Control, plan::Plan, templates::Templates};
use anyhow::Result;

pub(crate) fn write_control(
    plan: &mut Plan,
    build_dir: &str,
    package_name: &str,
    arch: &str,
    control: Control,
    templates: &Templates,
) -> Result<()> {
    plan.push(
        Action::cmd()
            .exe("mkdir")
            .arg("-p")
            .arg(format!("{build_dir}/debian"))
            .finish(),
    );

    let contents = templates.control(
        package_name,
        arch,
        &control.dependencies,
        &control.description,
    )?;
    plan.push(
        Action::write()
            .path(format!("{build_dir}/debian/control"))
            .contents(contents),
    );

    Ok(())
}
