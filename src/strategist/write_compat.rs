use crate::{
    action::macros::{cmd, write_file},
    plan::Plan,
};

pub(crate) fn write_compat(plan: &mut Plan, build_dir: &str) {
    plan.push(cmd!("mkdir", "-p", format!("{build_dir}/debian")));
    plan.push(write_file!(format!("{build_dir}/debian/compat"), "10"));
}
