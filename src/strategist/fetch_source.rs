use crate::{action::Action, config::Source, plan::Plan};

pub(crate) fn fetch_source(plan: &mut Plan, source: Source, build_dir: &str) {
    plan.push(Action::cmd().exe("mkdir").arg("-p").arg("/build").finish());

    match source {
        Source::None => {
            plan.push(Action::cmd().exe("mkdir").arg(build_dir).finish());
        }
        Source::GitClone {
            url,
            branch_or_tag,
            post_clone_scripts,
        } => {
            plan.push(
                Action::cmd()
                    .exe("git")
                    .arg("clone")
                    .arg(url)
                    .arg("--filter=blob:none")
                    .arg("--recursive")
                    .arg("--shallow-submodules")
                    .arg("--depth=1")
                    .arg("-q")
                    .arg(format!("--branch={}", branch_or_tag))
                    .arg(build_dir)
                    .finish(),
            );

            if let Some(post_clone_scripts) = post_clone_scripts {
                for script in post_clone_scripts {
                    let mut script = script.split(" ");
                    let exe = script.next().expect("script can't be empty");
                    let args = script.collect::<Vec<_>>();

                    plan.push(Action::cmd().exe(exe).args(args).finish());
                }
            }
        }
    }

    plan.push(
        Action::cmd()
            .exe("ls")
            .arg("-l")
            .arg(build_dir)
            .arg("--color=always")
            .finish(),
    );
}
