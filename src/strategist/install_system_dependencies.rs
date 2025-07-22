use crate::{action::Action, plan::Plan};

pub(crate) fn install_system_dependencies(plan: &mut Plan, dependencies: Vec<String>) {
    if dependencies.is_empty() {
        return;
    }

    plan.push(Action::cmd().exe("nala").arg("update").finish());

    let mut remaining = vec![];
    for dependency in dependencies {
        if dependency.starts_with("/") {
            plan.push(
                Action::cmd()
                    .exe("nala")
                    .arg("install")
                    .arg("--assume-yes")
                    .arg(dependency)
                    .finish(),
            );
        } else {
            remaining.push(dependency);
        }
    }

    plan.push(
        Action::cmd()
            .exe("nala")
            .arg("install")
            .arg("--assume-yes")
            .args(remaining)
            .finish(),
    );
}
