use anyhow::bail;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
pub(crate) enum Command {
    Parse,
    Explain,
    Build,

    PrintGitUrl,
    PrintGitTagOrBranch,

    BumpVersionTrailer,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "parse" => Self::Parse,
            "explain" => Self::Explain,
            "build" => Self::Build,
            "print-git-url" => Self::PrintGitUrl,
            "print-git-tag-or-branch" => Self::PrintGitTagOrBranch,
            "bump-version-trailer" => Self::BumpVersionTrailer,
            other => bail!("unknown command {other:?}"),
        })
    }
}

fn print_usage_and_exit() -> ! {
    const USAGE: &str = r#"Usage: build-deb-package <COMMAND> <CONFIG_PATH>

Commands:
  parse
  explain
  build
  print-git-url
  print-git-tag-or-branch
  bump-version-trailer
"#;

    eprintln!("{}", USAGE);
    std::process::exit(1);
}

pub(crate) fn parse() -> (Command, PathBuf) {
    let mut args = std::env::args();
    args.next();

    let Some(arg1) = args.next() else {
        print_usage_and_exit();
    };
    let cmd = Command::from_str(&arg1).unwrap_or_else(|err| {
        eprintln!("{err:?}");
        print_usage_and_exit()
    });

    let Some(arg2) = args.next() else {
        print_usage_and_exit();
    };
    let dir = if std::env::var("DOCKER").is_ok() {
        PathBuf::from("/shared")
    } else {
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("{err:?}");
            std::process::exit(1);
        })
    };
    let path = dir.join(arg2);

    (cmd, path)
}
