use crate::{commands::CommandExec, config::Config, strategist::Strategist};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Explain;

impl CommandExec for Explain {
    fn exec(self, config: Config) -> Result<()> {
        let plan = Strategist::make_plan(config)?;
        plan.explain();
        Ok(())
    }
}
