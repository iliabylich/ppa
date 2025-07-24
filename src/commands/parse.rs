use crate::{commands::CommandExec, config::Config};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Parse;

impl CommandExec for Parse {
    fn exec(self, config: Config) -> Result<()> {
        println!("{:#?}", config);
        Ok(())
    }
}
