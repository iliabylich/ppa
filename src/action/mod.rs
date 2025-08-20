pub(crate) use cmd::Cmd;
pub(crate) use cwd::Cwd;
use std::collections::HashMap;
pub(crate) use write_file::WriteFile;

mod cmd;
mod cwd;
pub(crate) mod macros;
mod write_file;

#[derive(Debug)]
pub(crate) enum Action {
    Cwd(Cwd),
    WriteFile(WriteFile),
    Cmd(Cmd),
}

impl Action {
    pub(crate) fn exec(&self, env: &HashMap<String, String>, path: &[String]) {
        match self {
            Self::Cwd(cwd) => cwd.exec(),
            Self::WriteFile(write) => write.exec(),
            Self::Cmd(cmd) => cmd.exec(env, path),
        }
    }

    pub(crate) fn explain(&self) {
        match self {
            Self::Cwd(cwd) => cwd.explain(),
            Self::WriteFile(write) => write.explain(),
            Self::Cmd(cmd) => cmd.explain(),
        }
    }
}
