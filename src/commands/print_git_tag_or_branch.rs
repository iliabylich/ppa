use crate::{
    commands::CommandExec,
    config::{Config, Source},
};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PrintGitTagOrBranch;

impl CommandExec for PrintGitTagOrBranch {
    fn exec(self, config: Config) -> Result<()> {
        let Source::GitClone { branch_or_tag, .. } = config.source else {
            println!("none");
            return Ok(());
        };
        println!("{branch_or_tag}");
        Ok(())
    }
}
