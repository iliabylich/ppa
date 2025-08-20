use crate::{action::macros::cmd, plan::Plan};

pub(crate) fn install_binstall_dependencies(plan: &mut Plan, binstall: Vec<String>) {
    if binstall.is_empty() {
        return;
    }

    plan.push(cmd!("cargo", "binstall", "-y", binstall));
}
