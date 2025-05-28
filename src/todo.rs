use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::Path;

const FILE_PATH: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub task: String,
    pub completed: bool,
}

pub fn read_todos() -> io::Result<Vec<TodoItem>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let todos: Vec<TodoItem> = serde_json::from_str(&data)?;
    Ok(todos)
}

pub fn write_todos(todos: &[TodoItem]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos)?;
    fs::write(FILE_PATH, json)?;
    Ok(())
}
