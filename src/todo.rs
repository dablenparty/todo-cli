use std::env::current_dir as current_working_dir;

use chrono::{DateTime, Local};
use crossterm::style::Stylize;
use inquire::formatter::MultiOptionFormatter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(debug_assertions)]
pub const TODO_FILE_NAME: &str = ".todos.debug.ron";
#[cfg(not(debug_assertions))]
pub const TODO_FILE_NAME: &str = ".todos.ron";

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct TodoItem {
    pub id: Uuid,
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    // TODO: due date?
}

impl PartialEq for TodoItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TodoItem {
    /// Provides a formatter for [`inquire::MultiSelect`] to display a list of todo items.
    /// This formatter will display only the short description of each item.
    pub const fn get_multi_select_formatter<'a>() -> MultiOptionFormatter<'a, TodoItem> {
        const FORMATTER: MultiOptionFormatter<'_, TodoItem> = &|l| {
            l.iter()
                .map(|t| t.value.short_desc.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };
        FORMATTER
    }

    #[inline]
    pub fn get_formatted_date(&self) -> String {
        self.created_at.format("%b %e, %Y %r").to_string()
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            short_desc: String::new(),
            long_desc: None,
            completed: false,
            created_at: Local::now(),
        }
    }
}

impl std::fmt::Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.short_desc.as_str();
        let styled = if self.completed {
            desc.green()
        } else {
            desc.red()
        };
        // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        write!(f, "{styled} ({})", self.get_formatted_date())
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
    let base_dir = current_working_dir()?;
    let todo_str = ron::to_string(todos)?;
    std::fs::write(base_dir.join(TODO_FILE_NAME), todo_str)?;
    Ok(())
}
