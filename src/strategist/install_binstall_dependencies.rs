use crate::{action::Action, plan::Plan};

pub(crate) fn install_binstall_dependencies(plan: &mut Plan, binstall: Vec<String>) {
    if binstall.is_empty() {
        return;
    }

    plan.push(
        Action::cmd()
            .exe("cargo")
            .arg("binstall")
            .arg("-y")
            .args(binstall)
            .finish(),
    );
}
