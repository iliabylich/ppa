pub(crate) use bash::{Bash, BashAddArg};
pub(crate) use cd::Cd;
pub(crate) use header::Header;
use std::collections::HashMap;
pub(crate) use write_file::WriteFile;

mod bash;
mod cd;
mod header;
mod write_file;

#[derive(Debug)]
pub(crate) enum Action {
    Cd(Cd),
    WriteFile(WriteFile),
    Bash(Bash),
    Header(Header),
}

impl Action {
    pub(crate) fn exec(&self, env: &HashMap<String, String>, path: &[String]) {
        match self {
            Self::Cd(cwd) => cwd.exec(),
            Self::WriteFile(write) => write.exec(),
            Self::Bash(cmd) => cmd.exec(env, path),
            Self::Header(_) => {}
        }
    }

    pub(crate) fn explain(&self) {
        match self {
            Self::Cd(cwd) => cwd.explain(),
            Self::WriteFile(write) => write.explain(),
            Self::Bash(cmd) => cmd.explain(),
            Self::Header(header) => header.explain(),
        }
    }
}
