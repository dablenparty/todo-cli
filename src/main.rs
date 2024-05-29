#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use inquire::Select;
use serde::{Deserialize, Serialize};
use strum::VariantArray;

mod cli;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct TodoItem {
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub completed: bool,
    // TODO: due date?
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Args::parse();
    println!("{cli:#?}");

    let command = if let Some(cmd) = cli.subcommand {
        cmd
    } else {
        Select::new("Select an option:", cli::Command::VARIANTS.to_vec()).prompt()?
    };

    command.handle_command();

    Ok(())
}
