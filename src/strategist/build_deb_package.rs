use crate::{action::Action, plan::Plan};

pub(crate) fn build_deb_package(plan: &mut Plan, build_dir: &str) {
    plan.push(Action::cwd(build_dir));
    plan.push(Action::cmd().exe("dh").arg("binary").finish());

    plan.push(
        Action::cmd()
            .exe("ls")
            .arg("-l")
            .arg("/build")
            .arg("--color=always")
            .finish(),
    );
}
