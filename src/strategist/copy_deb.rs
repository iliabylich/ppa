use crate::{action::Action, plan::Plan};

pub(crate) fn copy_deb(plan: &mut Plan, package_name: &str, version: &str, arch: &str) {
    let filename = format!("{package_name}_{version}_{arch}.deb");

    plan.push(
        Action::cmd()
            .exe("cp")
            .arg(format!("/build/{filename}"))
            .arg(format!("/shared/{filename}"))
            .finish(),
    );

    plan.push(
        Action::cmd()
            .exe("mkdir")
            .arg("-p")
            .arg("/shared/deb-latest")
            .finish(),
    );
    plan.push(
        Action::cmd()
            .exe("cp")
            .arg(format!("/build/{filename}"))
            .arg(format!("/shared/deb-latest/{package_name}.deb"))
            .finish(),
    );
}
