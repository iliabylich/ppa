use crate::{
    action::{Action, Bash, BashAddArg, Cd, Header, WriteFile},
    config::{Changelog, Config, GitClone, Source},
    nproc::num_cpus,
    plan::Plan,
};

pub struct Strategist;

impl Strategist {
    pub fn make_plan(config: Config) -> Plan {
        let Config {
            package_name,
            filepath: _filepath,
            version,
            dependencies,
            source,
            debian,
            arch,
            binstall,
            env,
            path,
            additionally_produced_packages,
        } = config;

        let mut plan = Plan::new(env, path);

        macro_rules! bash {
            ($exe:expr, $($x:expr),+ $(,)?) => {{
                let mut bash = Bash { exe: $exe.into(), args: vec![] };
                $(bash.add_arg($x);)+
                plan.push(Action::Bash(bash))
            }};
        }
        macro_rules! cd {
            ($dir:expr) => {
                plan.push(Action::Cd(Cd { dir: $dir.into() }))
            };
        }
        macro_rules! write_file {
            ($path:expr, $contents:expr) => {
                plan.push(Action::WriteFile(WriteFile {
                    path: $path.into(),
                    contents: $contents.into(),
                }))
            };
        }
        macro_rules! header {
            ($text:expr) => {
                plan.push(Action::Header(Header { text: $text.into() }))
            };
        }

        plan.add_env("DEB_BUILD_OPTIONS", format!("parallel={}", num_cpus()));

        header!("install system dependencies");
        if !dependencies.is_empty() {
            bash!("apt", "update");
            bash!("apt", "install", "-y", dependencies);
        }

        if !binstall.is_empty() {
            header!("binstall");
            bash!("cargo", "binstall", "-y", binstall);
        }

        header!("setup source directory");
        bash!("mkdir", "/build");
        let version = version.resolve();
        let build_dir = format!("/build/{}-{}", package_name, version);

        match source {
            Source::None => {
                bash!("mkdir", &build_dir);
            }
            Source::GitClone(GitClone {
                url,
                branch_or_tag,
                post_clone_scripts,
            }) => {
                const DEFAULT_GIT_CLONE_OPTIONS: &[&str] = &[
                    "--filter=blob:none",
                    "--recursive",
                    "--shallow-submodules",
                    "--depth=1",
                    "-q",
                ];

                bash!(
                    "git",
                    "clone",
                    url,
                    DEFAULT_GIT_CLONE_OPTIONS,
                    format!("--branch={}", branch_or_tag),
                    &build_dir
                );

                for script in post_clone_scripts {
                    bash!(script.exe, script.args);
                }
            }
        }
        bash!("ls", "-l", &build_dir, "--color=always");

        header!("setup debian directory");
        bash!("mkdir", "-p", format!("{build_dir}/debian"));
        write_file!(
            format!("{build_dir}/debian/changelog"),
            Changelog::render(&package_name, &version)
        );
        write_file!(format!("{build_dir}/debian/compat"), "10");
        write_file!(
            format!("{build_dir}/debian/control"),
            debian.control.render(&package_name, &arch)
        );
        if let Some(rules) = debian.rules {
            write_file!(format!("{build_dir}/debian/rules"), rules.render());
            bash!("chmod", "+x", format!("{build_dir}/debian/rules"));
        }

        header!("build deb package");
        cd!(build_dir);
        bash!("dh", "binary");
        bash!("ls", "-l", "/build", "--color=always");

        header!("copy deb package to output directory");
        let mut copy_deb = |package_name: &str| {
            let filename = format!("{package_name}_{version}_{arch}.deb");

            bash!(
                "cp",
                format!("/build/{filename}"),
                format!("/shared/{filename}")
            );

            bash!("mkdir", "-p", "/shared/deb-latest");
            bash!(
                "cp",
                format!("/build/{filename}"),
                format!("/shared/deb-latest/{package_name}.deb")
            );
        };
        copy_deb(&package_name);
        for extra_package_name in additionally_produced_packages {
            copy_deb(&extra_package_name);
        }

        plan
    }
}
