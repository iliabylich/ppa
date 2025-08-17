use crate::{action::Action, plan::Plan};

pub(crate) fn install_system_dependencies(plan: &mut Plan, dependencies: Vec<String>) {
    if dependencies.is_empty() {
        return;
    }

    plan.push(Action::cmd().exe("apt").arg("update").finish());

    plan.push(
        Action::cmd()
            .exe("apt")
            .arg("install")
            .arg("-y")
            .args(dependencies)
            .finish(),
    );
}
