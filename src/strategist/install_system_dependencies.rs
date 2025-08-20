use crate::{action::macros::cmd, plan::Plan};

pub(crate) fn install_system_dependencies(plan: &mut Plan, dependencies: Vec<String>) {
    if dependencies.is_empty() {
        return;
    }

    plan.push(cmd!("apt", "update"));

    plan.push(cmd!("apt", "install", "-y", dependencies));
}
