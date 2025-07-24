use crate::commands::Command;
use std::path::PathBuf;

fn print_usage_and_exit() -> ! {
    const USAGE: &str = r#"Usage: build-deb-package <COMMAND> <CONFIG_PATH{S}>...

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

pub(crate) fn parse() -> (Command, Vec<PathBuf>) {
    let mut args = std::env::args();
    args.next();

    let cmd = args.next().unwrap_or_else(|| print_usage_and_exit());
    let cmd = Command::parse(&cmd).unwrap_or_else(|| print_usage_and_exit());

    let dir = if std::env::var("DOCKER").is_ok() {
        PathBuf::from("/shared")
    } else {
        std::env::current_dir().unwrap_or_else(|err| {
            eprintln!("{err:?}");
            std::process::exit(1);
        })
    };

    let paths = args
        .filter(|filename| !filename.ends_with("Cargo.toml"))
        .filter(|filename| !filename.ends_with("config.toml"))
        .map(|filename| dir.join(filename))
        .collect::<Vec<_>>();

    (cmd, paths)
}
