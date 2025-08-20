use crate::{
    action::macros::{cmd, cwd},
    plan::Plan,
};

pub(crate) fn build_deb_package(plan: &mut Plan, build_dir: &str) {
    plan.push(cwd!(build_dir));
    plan.push(cmd!("dh", "binary"));

    plan.push(cmd!("ls", "-l", "/build", "--color=always"));
}
