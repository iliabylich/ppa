use crate::{
    colors::{GREEN, NC},
    plan::Plan,
};
use anyhow::{Context as _, Result};
use std::process::Command;

pub(crate) fn parallelize_build(plan: &mut Plan) -> Result<()> {
    let num_cpus = num_cpus()?;
    println!("{GREEN}Number of CPUs: {num_cpus}{NC}");

    plan.add_env("DEB_BUILD_OPTIONS", format!("parallel={}", num_cpus));
    Ok(())
}

fn num_cpus() -> Result<u8> {
    let stdout = Command::new("nproc")
        .output()
        .context("failed to call nproc")?
        .stdout;

    String::from_utf8(stdout)
        .context("non-utf-8 output of nproc")?
        .trim()
        .parse()
        .context("non-numeric output of nproc")
}
