use crate::{commands::CommandExec, config::Config, strategist::Strategist};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Build;

impl CommandExec for Build {
    fn exec(self, config: Config) -> Result<()> {
        let plan = Strategist::make_plan(config)?;
        plan.run()?;
        Ok(())
    }
}
