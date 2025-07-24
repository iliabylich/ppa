use anyhow::Result;
use config::Config;
use input::parse_input;
use templates::Templates;

use crate::commands::CommandExec;

mod action;
mod args;
mod colors;
mod commands;
mod config;
mod input;
mod plan;
mod strategist;
mod templates;

fn main() -> Result<()> {
    let (cmd, path) = args::parse();
    let configs = parse_input(path)?;

    for config in configs {
        cmd.exec(config)?;
    }

    Ok(())
}
