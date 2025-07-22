use crate::{num_cpus::num_cpus, plan::Plan};
use anyhow::Result;

pub(crate) fn parallelize_build(plan: &mut Plan) -> Result<()> {
    plan.add_env("DEB_BUILD_OPTIONS", format!("parallel={}", num_cpus()?));
    Ok(())
}
