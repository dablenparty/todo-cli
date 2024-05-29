#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use inquire::Select;
use strum::VariantArray;

mod cli;
mod todo;

fn main() -> anyhow::Result<()> {
    let cli = cli::Args::parse();
    println!("{cli:#?}");

    let command = if let Some(cmd) = cli.subcommand {
        cmd
    } else {
        Select::new("Select an option:", cli::Command::VARIANTS.to_vec()).prompt()?
    };

    command.handle_command()
}
