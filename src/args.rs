#[derive(Debug)]
pub(crate) enum Args {
    Parse,
    Explain,
    Build,

    PrintGitUrl,
    PrintGitTagOrBranch,

    BumpVersionTrailer,
}

const USAGE: &str = r#"Usage: build-deb-package <COMMAND>

Commands:
  parse
  explain
  build
  print-git-url
  print-git-tag-or-branch
  bump-version-trailer
"#;

impl Args {
    pub(crate) fn parse() -> Self {
        let mut args = std::env::args();
        args.next();
        if let Some(arg1) = args.next() {
            match &arg1[..] {
                "parse" => return Self::Parse,
                "explain" => return Self::Explain,
                "build" => return Self::Build,
                "print-git-url" => return Self::PrintGitUrl,
                "print-git-tag-or-branch" => return Self::PrintGitTagOrBranch,
                "bump-version-trailer" => return Self::BumpVersionTrailer,
                _ => {}
            }
        }

        eprintln!("{}", USAGE);
        std::process::exit(1);
    }
}
