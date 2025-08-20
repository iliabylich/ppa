use crate::{error, green, plan::Plan};
use std::process::Command;

pub(crate) fn parallelize_build(plan: &mut Plan) {
    let num_cpus = num_cpus();
    green!("Number of CPUs: {num_cpus}");

    plan.add_env("DEB_BUILD_OPTIONS", format!("parallel={}", num_cpus));
}

fn num_cpus() -> u8 {
    let stdout = Command::new("nproc")
        .output()
        .unwrap_or_else(|err| error!(err = err, "failed to call nproc"))
        .stdout;

    String::from_utf8(stdout)
        .unwrap_or_else(|err| error!(err = err, "non-utf-8 output of nproc"))
        .trim()
        .parse()
        .unwrap_or_else(|err| error!(err = err, "non-numeric output of nproc"))
}
