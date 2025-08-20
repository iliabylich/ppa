use crate::{
    colors::{GREEN, RED, NC, YELLOW},
    commands::CommandExec,
    config::{Config, Source},
};
use anyhow::{Context as _, Result, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CheckUpdates;

impl CommandExec for CheckUpdates {
    fn exec_each(self, configs: Vec<Config>) -> Result<()> {
        let results = configs.par_iter().map(exec).collect::<Vec<_>>();
        for lines in results {
            for line in lines {
                println!("{line}");
            }

            println!();
        }

        Ok(())
    }

    fn exec(self, _config: Config) -> Result<()> {
        unimplemented!("custom exec_many is used instead")
    }
}

fn exec(config: &Config) -> Vec<String> {
    let mut out = vec![];

    let package = &config.package_name;

    let Source::GitClone {
        url, branch_or_tag, ..
    } = &config.source
    else {
        out.push(format!(
            "[{package}] {GREEN}skipping, not a git source{NC}"
        ));
        return out;
    };

    out.push(format!("[{package}] {YELLOW}GitHub: {url}{NC}"));
    if !branch_or_tag.bytes().any(|b| b.is_ascii_digit()) {
        out.push(format!(
            "[{package}] {GREEN}skipping, non-numeric version {branch_or_tag}{NC}"
        ));
        return out;
    }

    let remote_version = match latest_release(url) {
        Ok(version) => version,
        Err(err1) => match latest_tag(url) {
            Ok(version) => version,
            Err(err2) => {
                out.push(format!(
                    "[{package}] {RED}failed to retrieve version{NC}"
                ));
                out.push(format!("[{package}] Release check error: {err1:?}"));
                out.push(format!("[{package}] Tag check error: {err2:?}"));
                return out;
            }
        },
    };

    out.push(format!(
        "[{package}] {YELLOW}Latest remote tag: {remote_version}{NC}"
    ));

    if &remote_version == branch_or_tag {
        out.push(format!("[{package}] {GREEN}NO UPDATES{NC}"));
    } else {
        out.push(format!(
            "[{package}] {RED}UPDATE AVAILABLE {branch_or_tag} -> {remote_version}{NC}"
        ));
    }

    out
}

fn latest_release(url: &str) -> Result<String> {
    gh([
        "release", "view", "-R", url, "--json", "tagName", "--jq", ".tagName",
    ])
}

fn latest_tag(url: &str) -> Result<String> {
    let owner_repo = url
        .strip_prefix("https://github.com/")
        .context("not a GitHub source")?
        .strip_suffix(".git")
        .context("no .git suffix")?;

    let stdout = gh([
        "api",
        "-H",
        "Accept: application/vnd.github+json",
        "-H",
        "X-GitHub-Api-Version: 2022-11-28",
        &format!("/repos/{owner_repo}/tags"),
        "--jq",
        ".[] | .name ",
    ])?;

    stdout
        .lines()
        .max()
        .map(|v| v.to_string())
        .context("no versions")
}

fn gh(args: impl IntoIterator<Item = impl Into<String>>) -> Result<String> {
    let args: Vec<String> = args.into_iter().map(|e| e.into()).collect();
    let output = std::process::Command::new("gh")
        .args(&args)
        .output()
        .with_context(|| format!("failed to exec gh {args:?}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("{}", stderr.trim());
    }

    let stdout = String::from_utf8(output.stdout).context("nont-utf8 output")?;

    Ok(stdout.trim().to_string())
}
