use crate::{Config, Templates, plan::Plan};
use anyhow::Result;
use build_deb_package::build_deb_package;
use copy_deb::copy_deb;
use fetch_source::fetch_source;
use install_binstall_dependencies::install_binstall_dependencies;
use install_system_dependencies::install_system_dependencies;
use parallelize_build::parallelize_build;
use resolve_version::resolve_version;
use write_changelog::write_changelog;
use write_compat::write_compat;
use write_control::write_control;
use write_rules::write_rules;

mod build_deb_package;
mod copy_deb;
mod fetch_source;
mod install_binstall_dependencies;
mod install_system_dependencies;
mod parallelize_build;
mod resolve_version;
mod write_changelog;
mod write_compat;
mod write_control;
mod write_rules;

pub(crate) struct Strategist;

impl Strategist {
    pub(crate) fn make_plan(config: Config) -> Result<Plan> {
        let Config {
            package_name,
            absolute_config_path: _absolute_config_path,
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

        let version = resolve_version(version);
        let build_dir = format!("/build/{}-{}", package_name, version);
        let templates = Templates::new()?;

        parallelize_build(&mut plan)?;
        install_system_dependencies(&mut plan, dependencies);
        install_binstall_dependencies(&mut plan, binstall);
        fetch_source(&mut plan, source, &build_dir);
        if debian.changelog {
            write_changelog(&mut plan, &build_dir, &package_name, &version, &templates)?;
        }
        if let Some(compat) = debian.compat {
            write_compat(&mut plan, &build_dir, compat, &templates)?;
        }
        if let Some(control) = debian.control {
            write_control(
                &mut plan,
                &build_dir,
                &package_name,
                &arch,
                control,
                &templates,
            )?;
        }
        if let Some(targets) = debian.rules {
            write_rules(&mut plan, &build_dir, targets, &templates)?;
        }
        build_deb_package(&mut plan, &build_dir);
        copy_deb(&mut plan, &package_name, &version, &arch);

        if let Some(package_names) = additionally_produced_packages {
            for package_name in package_names {
                copy_deb(&mut plan, &package_name, &version, &arch);
            }
        }

        Ok(plan)
    }
}
