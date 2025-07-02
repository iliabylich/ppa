use crate::colors::{GREEN, RESET, YELLOW};
use anyhow::{Context as _, Result, bail};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::Stdio,
};

#[derive(Debug)]
pub(crate) struct Plan {
    actions: Vec<Action>,
    env: HashMap<String, String>,
    path: Vec<String>,
}

impl Plan {
    pub(crate) fn new(env: HashMap<String, String>, path: Vec<String>) -> Self {
        Self {
            actions: vec![],
            env,
            path,
        }
    }

    pub(crate) fn exec(
        &mut self,
        exe: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.actions.push(Action::Script {
            exe: exe.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
        });
    }

    #[allow(dead_code)]
    pub(crate) fn exec1(
        &mut self,
        exe: impl Into<String>,
        arg1: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.exec(
            exe,
            [arg1.into()]
                .into_iter()
                .chain(args.into_iter().map(|e| e.into())),
        )
    }

    pub(crate) fn exec2(
        &mut self,
        exe: impl Into<String>,
        arg1: impl Into<String>,
        arg2: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.exec(
            exe,
            [arg1.into(), arg2.into()]
                .into_iter()
                .chain(args.into_iter().map(|e| e.into())),
        )
    }

    pub(crate) fn write_file(&mut self, path: impl Into<String>, contents: impl Into<String>) {
        self.actions.push(Action::WriteFile {
            path: path.into(),
            contents: contents.into(),
        })
    }

    pub(crate) fn cwd(&mut self, path: impl Into<String>) {
        self.actions
            .push(Action::ChangeWorkingDir { path: path.into() })
    }

    pub(crate) fn explain(self) {
        if !self.env.is_empty() {
            println!("{GREEN}ENV:{RESET}");
            for (key, val) in self.env {
                println!("{YELLOW}{key}={val}{RESET}");
            }
            println!();
        }

        if !self.path.is_empty() {
            println!("{GREEN}PATH (additional):{RESET}");
            for path in self.path {
                println!("{YELLOW}{path}{RESET}");
            }
            println!();
        }

        for script in self.actions {
            script.explain();
            println!();
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        for script in self.actions {
            println!("::group::{}", script.header());
            let result = script.run(&self.env, &self.path);
            println!("::endgroup::");
            result?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Action {
    ChangeWorkingDir { path: String },
    Script { exe: String, args: Vec<String> },
    WriteFile { path: String, contents: String },
}

impl Action {
    fn explain(&self) {
        println!("{}", self.explanation());
    }

    fn explanation(&self) -> String {
        match self {
            Self::ChangeWorkingDir { path } => format!("{GREEN}cwd {path}{RESET}"),
            Self::Script { exe, args } => format!("{GREEN}{} {}{RESET}", exe, args.join(" ")),
            Self::WriteFile { path, contents } => {
                format!("{YELLOW}Writing to {path}:{RESET}\n{contents}")
            }
        }
    }

    fn header(&self) -> String {
        match self {
            Action::ChangeWorkingDir { path } => format!("cwd to {path}"),
            Action::Script { exe, args } => format!("running {} {}", exe, args.join(" ")),
            Action::WriteFile { path, .. } => format!("writing to {path}"),
        }
    }

    fn run(self, env: &HashMap<String, String>, path: &[String]) -> Result<()> {
        self.explain();

        match self {
            Self::ChangeWorkingDir { path } => cwd(path),
            Self::Script { exe, args } => spawn_and_forward_stdout_and_stderr(exe, args, env, path),
            Self::WriteFile { path, contents } => write_file(path, contents),
        }
    }
}

fn cwd(path: String) -> Result<()> {
    std::env::set_current_dir(&path)
        .with_context(|| format!("failed to change working directory to {path}"))?;

    Ok(())
}

// this lint is false-positive, there's a wait() call in the statement below
#[expect(clippy::zombie_processes)]
fn spawn_and_forward_stdout_and_stderr(
    exe: String,
    args: Vec<String>,
    env: &HashMap<String, String>,
    path: &[String],
) -> Result<()> {
    let mut command = std::process::Command::new(exe.clone());

    let mut new_path = std::env::var("PATH").unwrap();
    for path in path {
        new_path = format!("{new_path}:{path}");
    }
    command.env("PATH", new_path);
    for (key, val) in env {
        command.env(key, val);
    }
    command.args(args.clone());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let child_stdout = child
        .stdout
        .take()
        .with_context(|| format!("failed to get child's stdout of {} {:?}", exe, args))?;

    let child_stderr = child
        .stderr
        .take()
        .with_context(|| format!("failed to get child's stderr of {} {:?}", exe, args))?;

    let (stdout_tx, _stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, _stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = std::thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            println!("{}", line);
            stdout_tx.send(line).unwrap();
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stderr_tx.send(line).unwrap();
        }
    });

    let status = child.wait().context("failed to wait on child")?;

    if !status.success() {
        bail!(
            "failed to execute {} {:?}\nstatus code: {:?}",
            exe,
            args,
            status.code()
        );
    }

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}

fn write_file(path: String, contents: String) -> Result<()> {
    std::fs::write(&path, contents).with_context(|| format!("Failed to write to {path}"))
}
