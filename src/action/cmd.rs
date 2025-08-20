use crate::{
    action::Exec,
    colors::{GREEN, NC},
};
use anyhow::{Context as _, Result, bail};
use std::{
    collections::HashMap,
    io::{BufRead as _, BufReader},
    process::Stdio,
};

#[derive(Debug)]
pub(crate) struct Cmd {
    exe: String,
    args: Vec<String>,
}

impl Cmd {
    pub(crate) fn new(exe: String, args: Vec<String>) -> Self {
        Self { exe, args }
    }
}

impl Exec for Cmd {
    fn exec(&self, env: &HashMap<String, String>, path: &[String]) -> Result<()> {
        let mut command = std::process::Command::new(&self.exe);

        let mut new_path = std::env::var("PATH").unwrap();
        for path in path {
            new_path = format!("{new_path}:{path}");
        }
        command.env("PATH", new_path);
        for (key, val) in env {
            command.env(key, val);
        }
        command.args(&self.args);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().unwrap();

        let stdout = child.stdout.take().expect("failed to get child's stdout");
        let stderr = child.stderr.take().expect("failed to get child's stderr");

        let stdout_thread = std::thread::spawn(move || {
            let stdout_lines = BufReader::new(stdout).lines();
            for line in stdout_lines {
                let line = line.unwrap();
                println!("{}", line);
            }
        });

        let stderr_thread = std::thread::spawn(move || {
            let stderr_lines = BufReader::new(stderr).lines();
            for line in stderr_lines {
                let line = line.unwrap();
                eprintln!("{}", line);
            }
        });

        let status = child.wait().context("failed to wait on child")?;

        if !status.success() {
            bail!(
                "failed to execute {} {:?}\nstatus code: {:?}",
                self.exe,
                self.args,
                status.code()
            );
        }

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();

        Ok(())
    }

    fn explanation(&self) -> String {
        format!("{GREEN}{} {}{NC}", self.exe, self.args.join(" "))
    }
}
