use crate::{action::macros::cmd, plan::Plan};

pub(crate) fn copy_deb(plan: &mut Plan, package_name: &str, version: &str, arch: &str) {
    let filename = format!("{package_name}_{version}_{arch}.deb");

    plan.push(cmd!(
        "cp",
        format!("/build/{filename}"),
        format!("/shared/{filename}")
    ));

    plan.push(cmd!("mkdir", "-p", "/shared/deb-latest"));
    plan.push(cmd!(
        "cp",
        format!("/build/{filename}"),
        format!("/shared/deb-latest/{package_name}.deb")
    ));
}
