use crate::{
    action::macros::cmd,
    config::{GitClone, Source},
    plan::Plan,
};

pub(crate) fn fetch_source(plan: &mut Plan, source: Source, build_dir: &str) {
    plan.push(cmd!("mkdir", "-p", "/build"));

    match source {
        Source::None => {
            plan.push(cmd!("mkdir", build_dir));
        }
        Source::GitClone(GitClone {
            url,
            branch_or_tag,
            post_clone_scripts,
        }) => {
            plan.push(cmd!(
                "git",
                "clone",
                url,
                "--filter=blob:none",
                "--recursive",
                "--shallow-submodules",
                "--depth=1",
                "-q",
                format!("--branch={}", branch_or_tag),
                build_dir
            ));

            for script in post_clone_scripts {
                let mut script = script.split(" ");
                let exe = script.next().expect("script can't be empty");
                let args = script.collect::<Vec<_>>();

                plan.push(cmd!(exe, args));
            }
        }
    }

    plan.push(cmd!("ls", "-l", build_dir, "--color=always"));
}
