use std::{
    collections::HashMap,
    io::{BufRead as _, BufReader},
    process::Stdio,
};

#[derive(Debug)]
pub(crate) struct Bash {
    pub(crate) exe: String,
    pub(crate) args: Vec<String>,
}

impl Bash {
    pub(crate) fn exec(&self, env: &HashMap<String, String>, path: &[String]) {
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

        let status = child
            .wait()
            .unwrap_or_else(|err| error!(err = err, "failed to wait on child"));

        if !status.success() {
            error!(
                "failed to execute {} {:?}\nstatus code: {:?}",
                self.exe,
                self.args,
                status.code()
            );
        }

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();
    }

    pub(crate) fn explain(&self) {
        green!("{} {}", self.exe, self.args.join(" "))
    }
}

pub(crate) trait BashAddArg<T> {
    fn add_arg(&mut self, data: T);
}

impl BashAddArg<String> for Bash {
    fn add_arg(&mut self, s: String) {
        self.args.push(s);
    }
}

impl BashAddArg<&String> for Bash {
    fn add_arg(&mut self, s: &String) {
        self.add_arg(s.clone());
    }
}

impl BashAddArg<Vec<String>> for Bash {
    fn add_arg(&mut self, strings: Vec<String>) {
        for s in strings {
            self.add_arg(s);
        }
    }
}

impl BashAddArg<&str> for Bash {
    fn add_arg(&mut self, s: &str) {
        self.add_arg(s.to_string());
    }
}

impl BashAddArg<Vec<&str>> for Bash {
    fn add_arg(&mut self, strings: Vec<&str>) {
        for s in strings {
            self.add_arg(s);
        }
    }
}

impl BashAddArg<&[&str]> for Bash {
    fn add_arg(&mut self, strings: &[&str]) {
        for s in strings {
            self.add_arg(*s);
        }
    }
}
