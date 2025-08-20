use crate::{config::Config, plan::Plan};
use build_deb_package::build_deb_package;
use copy_deb::copy_deb;
use fetch_source::fetch_source;
use install_binstall_dependencies::install_binstall_dependencies;
use install_system_dependencies::install_system_dependencies;
use parallelize_build::parallelize_build;
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
mod write_changelog;
mod write_compat;
mod write_control;
mod write_rules;

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

        let version = version.resolve();
        let build_dir = format!("/build/{}-{}", package_name, version);

        parallelize_build(&mut plan);
        install_system_dependencies(&mut plan, dependencies);
        install_binstall_dependencies(&mut plan, binstall);
        fetch_source(&mut plan, source, &build_dir);
        write_changelog(&mut plan, &build_dir, &package_name, &version);
        write_compat(&mut plan, &build_dir);
        write_control(&mut plan, &build_dir, &package_name, &arch, debian.control);
        write_rules(&mut plan, &build_dir, debian.rules);
        build_deb_package(&mut plan, &build_dir);
        copy_deb(&mut plan, &package_name, &version, &arch);

        for package_name in additionally_produced_packages {
            copy_deb(&mut plan, &package_name, &version, &arch);
        }

        plan
    }
}
