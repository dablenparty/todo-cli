use clap::{Parser, Subcommand};
use inquire::Text;
use strum::VariantArray;

use crate::todo;

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
    pub fn handle_command(self) -> anyhow::Result<()> {
        match self {
            Command::Add => handle_add(),
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

fn handle_add() -> anyhow::Result<()> {
    let short_desc = Text::new("What do you need to do?").prompt()?;
    let long_desc = Text::new("Any additional details?").prompt_skippable()?;
    let todo = todo::TodoItem {
        short_desc,
        long_desc,
        completed: false,
    };

    let mut todos = todo::read_todo_file()?;
    todos.push(todo);
    todo::write_todo_file(&todos)?;

    println!("Todo added successfully!");
    Ok(())
}
