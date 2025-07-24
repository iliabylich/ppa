use crate::{
    commands::CommandExec,
    config::{Config, Source},
};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PrintGitUrl;

impl CommandExec for PrintGitUrl {
    fn exec(self, config: Config) -> Result<()> {
        let Source::GitClone { url, .. } = config.source else {
            println!("none");
            return Ok(());
        };
        println!("{url}");
        Ok(())
    }
}
