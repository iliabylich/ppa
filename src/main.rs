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
    let (cmd, paths) = args::parse();

    let mut configs = vec![];
    for path in paths {
        configs.append(&mut parse_input(path)?);
    }

    cmd.exec_each(configs)?;

    Ok(())
}
