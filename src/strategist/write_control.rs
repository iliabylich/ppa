use crate::{
    action::macros::{cmd, write_file},
    config::Control,
    plan::Plan,
};

pub(crate) fn write_control(
    plan: &mut Plan,
    build_dir: &str,
    package_name: &str,
    arch: &str,
    control: Control,
) {
    plan.push(cmd!("mkdir", "-p", format!("{build_dir}/debian")));

    let dependencies = control.dependencies.join(", ");
    let description = &control.description;

    plan.push(write_file!(
        format!("{build_dir}/debian/control"),
        "Source: {package_name}
Section: utils
Priority: extra
Maintainer: John Doe <john@doe.org>
Standards-Version: 4.6.2

Package: {package_name}
Section: utils
Priority: extra
Architecture: {arch}
Depends: {dependencies}
Description: {description}
",
    ));
}
