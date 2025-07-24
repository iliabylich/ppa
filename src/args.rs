use crate::commands::Command;
use std::path::PathBuf;

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
    let cmd = Command::parse(&arg1).unwrap_or_else(|| print_usage_and_exit());

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
