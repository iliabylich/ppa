use crate::config::Config;
use anyhow::Result;
use build::Build;
use bump_version_trailer::BumpVersionTrailer;
use explain::Explain;
use parse::Parse;
use print_git_tag_or_branch::PrintGitTagOrBranch;
use print_git_url::PrintGitUrl;

mod build;
mod bump_version_trailer;
mod explain;
mod parse;
mod print_git_tag_or_branch;
mod print_git_url;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Command {
    Parse(Parse),
    Explain(Explain),
    Build(Build),

    PrintGitUrl(PrintGitUrl),
    PrintGitTagOrBranch(PrintGitTagOrBranch),

    BumpVersionTrailer(BumpVersionTrailer),
}

pub(crate) trait CommandExec {
    fn exec(self, config: Config) -> Result<()>;
}

impl CommandExec for Command {
    fn exec(self, config: Config) -> Result<()> {
        match self {
            Command::Parse(inner) => inner.exec(config),
            Command::Explain(inner) => inner.exec(config),
            Command::Build(inner) => inner.exec(config),
            Command::PrintGitUrl(inner) => inner.exec(config),
            Command::PrintGitTagOrBranch(inner) => inner.exec(config),
            Command::BumpVersionTrailer(inner) => inner.exec(config),
        }
    }
}

impl Command {
    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            "parse" => Some(Self::Parse(Parse)),
            "explain" => Some(Self::Explain(Explain)),
            "build" => Some(Self::Build(Build)),
            "print-git-url" => Some(Self::PrintGitUrl(PrintGitUrl)),
            "print-git-tag-or-branch" => Some(Self::PrintGitTagOrBranch(PrintGitTagOrBranch)),
            "bump-version-trailer" => Some(Self::BumpVersionTrailer(BumpVersionTrailer)),
            _ => None,
        }
    }
}
