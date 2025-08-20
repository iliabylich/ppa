use build_deb_package::{args::paths_from_args, config::Config, green, red};
use std::{collections::VecDeque, sync::Mutex};

fn main() {
    let sources = paths_from_args()
        .into_iter()
        .map(Config::read)
        .filter_map(|config| config.into_git_repo_and_version())
        .map(|(repo, version)| GitSource { repo, version })
        .collect::<VecDeque<_>>();

    let queue = Mutex::new(sources);

    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| worker(&queue));
        }
    })
}

#[derive(Debug)]
struct GitSource {
    repo: String,
    version: String,
}

impl GitSource {
    fn latest_tag(&self) -> Result<String, String> {
        let stdout = Self::gh([
            "api",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "X-GitHub-Api-Version: 2022-11-28",
            &format!("/repos/{}/tags", self.repo),
            "--jq",
            ".[] | .name ",
        ])?;

        stdout
            .lines()
            .max()
            .map(|v| v.to_string())
            .ok_or_else(|| "no tags".to_string())
    }

    fn latest_release(&self) -> Result<String, String> {
        Self::gh([
            "release", "view", "-R", &self.repo, "--json", "tagName", "--jq", ".tagName",
        ])
    }

    fn latest_remote_version(&self) -> Result<String, String> {
        match self.latest_release() {
            Ok(release) => Ok(release),
            Err(release_check_err) => match self.latest_tag() {
                Ok(tag) => Ok(tag),
                Err(tag_check_err) => Err(format!(
                    "release check failed: {release_check_err}\ntag check failed: {tag_check_err}"
                )),
            },
        }
    }

    fn check(self) -> CheckResult {
        let up_to_date = self
            .latest_remote_version()
            .map(|remote_version| (remote_version == self.version, remote_version));

        CheckResult {
            repo: self.repo,
            local_version: self.version,
            up_to_date,
        }
    }

    fn gh(args: impl IntoIterator<Item = impl Into<String>>) -> Result<String, String> {
        let args: Vec<String> = args.into_iter().map(|e| e.into()).collect();
        let output = std::process::Command::new("gh")
            .args(&args)
            .output()
            .map_err(|err| format!("failed to exec gh {args:?}: {err:?}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("gh returned an error:\n{}", stderr.trim()));
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|err| format!("non-utf8 output from gh: {err:?}"))?;

        Ok(stdout.trim().to_string())
    }
}

struct CheckResult {
    repo: String,
    local_version: String,
    up_to_date: Result<(bool, String), String>,
}

fn worker(queue: &Mutex<VecDeque<GitSource>>) {
    loop {
        let mut guard = queue.lock().unwrap();
        let Some(source) = guard.pop_front() else {
            break;
        };
        drop(guard);

        let CheckResult {
            repo,
            local_version,
            up_to_date,
        } = source.check();

        match up_to_date {
            Ok((true, remote_version)) => {
                green!("[{repo}] up to date: {local_version} vs {remote_version}")
            }
            Ok((false, remote_version)) => {
                red!("[{repo}] outdated: {local_version} vs {remote_version}")
            }
            Err(err) => {
                red!("[{repo}] failed to load info:\n{err}")
            }
        }
    }
}
