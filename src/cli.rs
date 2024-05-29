use clap::{Parser, Subcommand};
use strum::VariantArray;

/// This is a simple CLI tool to manage a list of todos.
/// Running with no arguments will run in interactive mode.
/// Subcommands are provided for quick access to specific actions.
#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(Debug, Clone, Copy, Subcommand, VariantArray, strum::Display)]
pub enum Command {
    /// Add a new todo to the list
    Add,
    /// Edit an existing todo
    Edit,
    /// Remove a todo from the list
    #[command(visible_alias = "rm")]
    Remove,
    /// List all todos or apply filters
    #[command(visible_alias = "ls")]
    List,
}

impl Command {
    pub fn handle_command(self) {
        match self {
            Command::Add => {
                todo!("Adding a new todo");
            }
            Command::Edit => {
                todo!("Editing an existing todo");
            }
            Command::Remove => {
                todo!("Removing a todo");
            }
            Command::List => {
                todo!("Listing all todos");
            }
        }
    }
}
