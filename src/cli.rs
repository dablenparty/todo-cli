use clap::{Parser, Subcommand};

/// This is a simple CLI tool to manage a list of todos.
/// Running with no arguments will run in interactive mode.
/// Subcommands are provided for quick access to specific actions.
#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: Option<CliCommand>,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
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
