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

#[derive(Debug, Default, Clone, clap::Args)]
pub struct EditArgs {
    /// Edit the full todo item. If not set, only the completion status will be edited.
    #[arg(short, long = "full")]
    pub full_edit: bool,
}

#[derive(Debug, Default, Clone, clap::Args)]
pub struct RemoveArgs {
    /// Remove all todos.
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Clone, Subcommand, EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray, strum::Display))]
pub enum Command {
    /// Add a new todo to the list
    Add(AddArgs),
    /// Edit an existing todo
    Edit(EditArgs),
    /// Remove a todo from the list
    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),
    /// List all todos or apply filters
    #[command(visible_alias = "ls")]
    List,
}

impl From<CommandDiscriminants> for Command {
    fn from(disc: CommandDiscriminants) -> Self {
        match disc {
            CommandDiscriminants::Add => Command::Add(AddArgs::default()),
            CommandDiscriminants::Edit => Command::Edit(EditArgs::default()),
            CommandDiscriminants::Remove => Command::Remove(RemoveArgs::default()),
            CommandDiscriminants::List => Command::List,
        }
    }
}

impl Command {
    /// Handle the command and perform the necessary actions.
    ///
    /// # Errors
    ///
    /// This function will propagate any errors that occur during the handling of the command.
    pub fn handle_command(self) -> anyhow::Result<()> {
        match self {
            Command::Add(args) => handle_add(args),
            Command::Edit(args) => handle_edit(&args),
            Command::Remove(args) => handle_remove(&args),
            Command::List => handle_list(),
        }
    }
}

/// Handle the remove command. This will prompt the user to select todos to remove.
/// The user will be prompted to confirm the removal before the todos are removed.
///
/// # Arguments
///
/// * `args` - The arguments provided to the remove command.
///
/// # Errors
///
/// If the todos cannot be read or written, an error will be returned.
fn handle_remove(args: &RemoveArgs) -> Result<(), anyhow::Error> {
    let mut todos = todo::read_todo_file()?;

    if args.all {
        let confirm = Confirm::new(&format!(
            "Are you sure you want to remove {} todos?",
            todos.len()
        ))
        .with_default(false)
        .prompt()?;
        if confirm {
            todos.clear();
        }
    } else {
        let selected_ids: HashSet<Uuid> = loop {
            let selections = MultiSelect::new("Select todos to remove:", todos.clone()).prompt()?;
            if selections.is_empty() {
                return Ok(());
            }
            let confirm = Confirm::new("Are you sure you want to remove these todos?")
                .with_default(false)
                .prompt()?;
            if confirm {
                break selections.iter().map(|t| t.id).collect();
            }
        };

        todos.retain(|t| !selected_ids.contains(&t.id));
    }

    todo::write_todo_file(&todos)?;
    println!("Todos removed successfully!");

    Ok(())
}

/// Handle the list command. This will read the todos from the file and print them to the console.
///
/// # Errors
///
/// If the todos cannot be read, an error will be returned.
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

/// Handle the edit command. Depending on the arguments, the user will be prompted to edit a single
/// todo or simply mark todos as complete. The todos will be written back to the file after editing.
///
/// # Arguments
///
/// * `args` - The arguments provided to the edit command.
///
/// # Errors
///
/// If the todos cannot be read or written, an error will be returned.
fn handle_edit(args: &EditArgs) -> Result<(), anyhow::Error> {
    let mut todos = todo::read_todo_file()?;

    if args.full_edit {
        full_edit(&mut todos)?;
    } else {
        quick_edit(&mut todos)?;
    };

    todo::write_todo_file(&todos)?;
    println!("Todos edited successfully!");

    Ok(())
}

fn quick_edit(todos: &mut [todo::TodoItem]) -> Result<(), anyhow::Error> {
    let completed_indices: Vec<_> = todos
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if t.completed { Some(i) } else { None })
        .collect();
    let selection = MultiSelect::new("Select todos to mark as complete:", todos.to_owned())
        .with_default(&completed_indices)
        .prompt()?;
    let selected_ids: HashSet<Uuid> = selection.iter().map(|i| i.id).collect();
    for todo in todos {
        todo.completed = selected_ids.contains(&todo.id);
    }
    Ok(())
}

fn full_edit(todos: &mut [todo::TodoItem]) -> Result<(), anyhow::Error> {
    let selection = Select::new("Select a todo:", todos.to_owned()).prompt()?;
    let short_desc = Text::new("What do you need to do?")
        .with_default(&selection.short_desc)
        .prompt()?;
    let long_desc = Text::new("Any additional details?")
        .with_default(selection.long_desc.as_deref().unwrap_or_default())
        .prompt_skippable()?;
    let completed = Confirm::new("Complete?")
        .with_default(selection.completed)
        .prompt()?;
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
        created_at: existing_todo.created_at,
    };
    todos[index] = updated_todo;
    Ok(())
}

/// Handle the add command. This will prompt the user for the necessary information to create a new
/// todo item. The todo will be written to the file after creation.
///
/// # Arguments
///
/// * `args` - The arguments provided to the add command.
///
/// # Errors
///
/// If the todos cannot be read or written, an error will be returned.
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
            short_desc,
            long_desc,
            ..Default::default()
        }
    };

    let mut todos = todo::read_todo_file()?;
    todos.push(todo);
    todo::write_todo_file(&todos)?;
    println!("Todo added successfully!");

    Ok(())
}
