use crate::{
    action::macros::{cmd, write_file},
    plan::Plan,
};

pub(crate) fn write_changelog(plan: &mut Plan, build_dir: &str, package_name: &str, version: &str) {
    plan.push(cmd!("mkdir", "-p", format!("{build_dir}/debian")));

    plan.push(write_file!(
        format!("{build_dir}/debian/changelog"),
        "{package_name} ({version}) unstable; urgency=low

  * Release

 -- John Doe <john@doe.org>  Wed, 22 May 2024 17:54:24 +0000
"
    ));
}
