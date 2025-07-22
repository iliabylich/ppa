use anyhow::{Result, bail};
use args::Args;
use config::Config;
use input::Input;
use list::List;
use strategist::Strategist;
use templates::Templates;

mod action;
mod args;
mod colors;
mod config;
mod input;
mod list;
mod plan;
mod strategist;
mod templates;

fn main() -> Result<()> {
    let args = Args::parse();
    let input = Input::from_env()?;

    match (args, input) {
        (Args::Parse, Input::Singular(config)) => {
            println!("{:#?}", config);
        }

        (Args::Parse, Input::Plural(list)) => {
            println!("{:#?}", list);
        }

        (Args::Explain, Input::Singular(config)) => {
            let plan = Strategist::make_plan(config)?;
            plan.explain();
        }

        (Args::Build, input) => {
            for config in input.expand_into_config_list()? {
                let plan = Strategist::make_plan(config)?;
                plan.run()?;
            }
        }

        (Args::PrintGitUrl, Input::Singular(config)) => {
            let git_url = config.source.git_url().unwrap_or("none");
            println!("{git_url}");
        }

        (Args::PrintGitTagOrBranch, Input::Singular(config)) => {
            let git_branch = config.source.git_branch_or_tag().unwrap_or("none");
            println!("{git_branch}");
        }

        (Args::BumpVersionTrailer, input) => {
            for config in input.expand_into_config_list()? {
                config.bump_version_trailer()?;
            }
        }

        (args, input) => {
            bail!("can't run {args:?} on '{input}'")
        }
    }

    Ok(())
}
