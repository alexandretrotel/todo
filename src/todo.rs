use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::Path;

const FILE_PATH: &str = "todo.json";

/// Represents a single to-do item.
///
/// # Fields
/// - `id`: A unique identifier for the to-do item.
/// - `task`: The description of the to-do task.
/// - `completed`: Indicates whether the task has been completed.
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub task: String,
    pub completed: bool,
}

/// Reads the list of to-do items from the JSON file.
///
/// If the file does not exist, returns an empty vector.
///
/// # Errors
/// Returns an `io::Error` if there is an error reading the file or parsing the JSON.
///
/// # Examples
/// ```
/// let todos = read_todos().expect("Failed to read todos");
/// println!("{:?}", todos);
/// ```
pub fn read_todos() -> io::Result<Vec<TodoItem>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let todos: Vec<TodoItem> = serde_json::from_str(&data)?;
    Ok(todos)
}

/// Writes the list of to-do items to the JSON file.
///
/// Overwrites the existing content with the provided list in pretty JSON format.
///
/// # Errors
/// Returns an `io::Error` if there is an error serializing the data or writing to the file.
///
/// # Examples
/// ```
/// let todos = vec![
///     TodoItem { id: 1, task: "Learn Rust".to_string(), completed: false }
/// ];
/// write_todos(&todos).expect("Failed to write todos");
/// ```
pub fn write_todos(todos: &[TodoItem]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos)?;
    fs::write(FILE_PATH, json)?;
    Ok(())
}
