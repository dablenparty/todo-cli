#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use inquire::Select;
use strum::VariantArray;

mod cli;
mod todo;

fn main() -> anyhow::Result<()> {
    let cli = cli::Args::parse();
    #[cfg(debug_assertions)]
    println!("{cli:#?}");

    let command = if let Some(cmd) = cli.subcommand {
        cmd
    } else {
        let disc = Select::new(
            "Select a command:",
            cli::CommandDiscriminants::VARIANTS.to_vec(),
        )
        .prompt()?;
        cli::Command::from(disc)
    };

    command.handle_command()
}
