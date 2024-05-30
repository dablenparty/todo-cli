use std::env::current_dir as current_working_dir;

use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const TODO_FILE_NAME: &str = ".todos.ron";

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct TodoItem {
    pub id: Uuid,
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub completed: bool,
    // TODO: due date?
}

impl std::fmt::Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.short_desc.as_str();
        let styled = if self.completed {
            desc.green()
        } else {
            desc.red()
        };
        write!(f, "{styled}")
    }
}

/// Read the todo file from the current directory. If the file does not exist, an empty `Vec` is returned.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the file is not valid RON.
pub fn read_todo_file() -> anyhow::Result<Vec<TodoItem>> {
    let todo_path = current_working_dir()?.join(TODO_FILE_NAME);
    if !todo_path.try_exists().unwrap_or_default() {
        return Ok(Vec::new());
    }
    let todo_str = std::fs::read_to_string(todo_path)?;
    let todos: Vec<TodoItem> = ron::from_str(&todo_str)?;
    Ok(todos)
}

/// Write the todos to the file in the current directory. This will overwrite the existing file.
/// If the file does not exist, it will be created.
///
/// # Errors
///
/// This function will return an error if the file cannot be written or if the todos cannot be serialized to RON.
pub fn write_todo_file(todos: &[TodoItem]) -> anyhow::Result<()> {
    let mut sorted = todos.to_owned();
    sorted.sort_by_key(|item| item.short_desc.clone());
    let base_dir = current_working_dir()?;
    let todo_str = ron::to_string(&sorted)?;
    std::fs::write(base_dir.join(TODO_FILE_NAME), todo_str)?;
    Ok(())
}
