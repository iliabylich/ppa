use anyhow::Result;
use args::Command;
use config::Config;
use input::parse_input;
use strategist::Strategist;
use templates::Templates;

mod action;
mod args;
mod colors;
mod config;
mod input;
mod plan;
mod strategist;
mod templates;

fn main() -> Result<()> {
    let (cmd, path) = args::parse();
    let configs = parse_input(path)?;

    for config in configs {
        match cmd {
            Command::Parse => {
                println!("{:#?}", config);
            }

            Command::Explain => {
                let plan = Strategist::make_plan(config)?;
                plan.explain();
            }

            Command::Build => {
                let plan = Strategist::make_plan(config)?;
                plan.run()?;
            }

            Command::PrintGitUrl => {
                let git_url = config.source.git_url().unwrap_or("none");
                println!("{git_url}");
            }

            Command::PrintGitTagOrBranch => {
                let git_branch = config.source.git_branch_or_tag().unwrap_or("none");
                println!("{git_branch}");
            }

            Command::BumpVersionTrailer => {
                config.bump_version_trailer()?;
            }
        }
    }

    Ok(())
}
