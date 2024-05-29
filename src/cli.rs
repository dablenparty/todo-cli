use anyhow::Context;
use clap::{Parser, Subcommand};
use inquire::{Confirm, Select, Text};
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
            Command::Edit => handle_edit(),
            Command::Remove => {
                todo!("Removing a todo");
            }
            Command::List => {
                todo!("Listing all todos");
            }
        }
    }
}

fn handle_edit() -> Result<(), anyhow::Error> {
    let mut todos = todo::read_todo_file()?;

    let selection = Select::new("Select a todo:", todos.clone()).prompt()?;
    let short_desc = Text::new("What do you need to do?")
        .with_default(&selection.short_desc)
        .prompt()?;
    let long_desc = Text::new("Any additional details?")
        .with_default(selection.long_desc.as_deref().unwrap_or_default())
        .prompt_skippable()?;
    let completed = Confirm::new("Complete?")
        .with_default(selection.completed)
        .prompt()?;

    // replace the selected todo with the updated one
    let index = todos
        .iter()
        .position(|t| t.id == selection.id)
        .context("Failed to relocate edited todo in memory")?;
    let existing_todo = todos.get(index).unwrap();
    let updated_todo = todo::TodoItem {
        id: existing_todo.id,
        short_desc,
        long_desc,
        completed,
    };
    todos[index] = updated_todo;

    todo::write_todo_file(&todos)?;
    Ok(())
}

fn handle_add() -> anyhow::Result<()> {
    let short_desc = Text::new("What do you need to do?").prompt()?;
    let long_desc = Text::new("Any additional details?").prompt_skippable()?;
    let todo = todo::TodoItem {
        id: uuid::Uuid::new_v4(),
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
