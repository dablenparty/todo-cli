use std::collections::HashSet;

use anyhow::Context;
use clap::{Parser, Subcommand};
use inquire::{Confirm, MultiSelect, Select, Text};
use strum::{EnumDiscriminants, VariantArray};
use uuid::Uuid;

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

#[derive(Debug, Default, Clone, clap::Args)]
pub struct AddArgs {
    pub short_desc: Option<String>,
}

#[derive(Debug, Clone, Subcommand, EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray, strum::Display))]
pub enum Command {
    /// Add a new todo to the list
    Add(AddArgs),
    /// Edit an existing todo
    Edit,
    /// Remove a todo from the list
    #[command(visible_alias = "rm")]
    Remove,
    /// List all todos or apply filters
    #[command(visible_alias = "ls")]
    List,
}

impl From<CommandDiscriminants> for Command {
    fn from(disc: CommandDiscriminants) -> Self {
        match disc {
            CommandDiscriminants::Add => Command::Add(AddArgs::default()),
            CommandDiscriminants::Edit => Command::Edit,
            CommandDiscriminants::Remove => Command::Remove,
            CommandDiscriminants::List => Command::List,
        }
    }
}

impl Command {
    pub fn handle_command(self) -> anyhow::Result<()> {
        match self {
            Command::Add(args) => handle_add(args),
            Command::Edit => handle_edit(),
            Command::Remove => handle_remove(),
            Command::List => handle_list(),
        }
    }
}

fn handle_remove() -> Result<(), anyhow::Error> {
    let mut todos = todo::read_todo_file()?;

    let selections: HashSet<Uuid> = MultiSelect::new("Select todos to remove:", todos.clone())
        .prompt()?
        .iter()
        .map(|i| i.id)
        .collect();

    todos.retain(|t| !selections.contains(&t.id));
    todo::write_todo_file(&todos)?;

    Ok(())
}

fn handle_list() -> Result<(), anyhow::Error> {
    let todos = todo::read_todo_file()?;
    if todos.is_empty() {
        println!("No todos found.");
    } else {
        for todo in &todos {
            println!("{todo}");
        }
    }
    Ok(())
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

fn handle_add(args: AddArgs) -> anyhow::Result<()> {
    let todo = if let Some(short_desc) = args.short_desc {
        todo::TodoItem {
            short_desc,
            ..Default::default()
        }
    } else {
        let short_desc = Text::new("What do you need to do?").prompt()?;
        let long_desc = Text::new("Any additional details?").prompt_skippable()?;
        todo::TodoItem {
            id: uuid::Uuid::new_v4(),
            short_desc,
            long_desc,
            completed: false,
        }
    };

    let mut todos = todo::read_todo_file()?;
    todos.push(todo);
    todo::write_todo_file(&todos)?;

    println!("Todo added successfully!");
    Ok(())
}
