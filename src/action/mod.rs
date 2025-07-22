use anyhow::Result;
use cmd::Cmd;
use cwd::Cwd;
use std::collections::HashMap;
use write::Write;

mod cmd;
mod cwd;
mod write;

pub(crate) trait Exec {
    fn exec(&self, env: &HashMap<String, String>, path: &[String]) -> Result<()>;
    fn explanation(&self) -> String;
}

#[derive(Debug)]
pub(crate) enum Action {
    Cwd(Cwd),
    Write(Write),
    Cmd(Cmd),
}

impl Exec for Action {
    fn exec(&self, env: &HashMap<String, String>, path: &[String]) -> Result<()> {
        match self {
            Self::Cwd(cwd) => cwd.exec(env, path),
            Self::Write(write) => write.exec(env, path),
            Self::Cmd(cmd) => cmd.exec(env, path),
        }
    }

    fn explanation(&self) -> String {
        match self {
            Self::Cwd(cwd) => cwd.explanation(),
            Self::Write(write) => write.explanation(),
            Self::Cmd(cmd) => cmd.explanation(),
        }
    }
}

impl Action {
    pub(crate) fn cwd(dir: impl Into<String>) -> Self {
        Self::Cwd(Cwd::new(dir.into()))
    }

    pub(crate) fn write() -> WriteActionBuilder<WriteActionBuilderEmptyState> {
        WriteActionBuilder {
            state: WriteActionBuilderEmptyState,
        }
    }

    pub(crate) fn cmd() -> CmdActionBuilder<CmdActionBuilderEmptyState> {
        CmdActionBuilder {
            state: CmdActionBuilderEmptyState,
        }
    }
}

pub(crate) struct WriteActionBuilder<S> {
    state: S,
}

pub(crate) struct WriteActionBuilderEmptyState;

impl WriteActionBuilder<WriteActionBuilderEmptyState> {
    pub(crate) fn path(self, path: impl Into<String>) -> WriteActionBuilder<WriteBuilderPathState> {
        WriteActionBuilder {
            state: WriteBuilderPathState { path: path.into() },
        }
    }
}

pub(crate) struct WriteBuilderPathState {
    path: String,
}

impl WriteActionBuilder<WriteBuilderPathState> {
    pub(crate) fn contents(self, contents: impl Into<String>) -> Action {
        Action::Write(Write::new(self.state.path, contents.into()))
    }
}

pub(crate) struct CmdActionBuilder<S> {
    state: S,
}

pub(crate) struct CmdActionBuilderEmptyState;

impl CmdActionBuilder<CmdActionBuilderEmptyState> {
    pub(crate) fn exe(self, exe: impl Into<String>) -> CmdActionBuilder<CmdActionBuilderExeState> {
        CmdActionBuilder {
            state: CmdActionBuilderExeState {
                exe: exe.into(),
                args: vec![],
            },
        }
    }
}

pub(crate) struct CmdActionBuilderExeState {
    exe: String,
    args: Vec<String>,
}

impl CmdActionBuilder<CmdActionBuilderExeState> {
    pub(crate) fn arg(mut self, arg: impl Into<String>) -> Self {
        self.state.args.push(arg.into());
        self
    }

    pub(crate) fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for arg in args {
            self.state.args.push(arg.into());
        }
        self
    }

    pub(crate) fn finish(self) -> Action {
        Action::Cmd(Cmd::new(self.state.exe, self.state.args))
    }
}
