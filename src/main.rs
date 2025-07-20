mod todo;
use clap::{Parser, Subcommand};
use inquire::{MultiSelect, Text};
use todo::{TodoItem, read_todos, write_todos};

/// Command-line interface for a simple to-do list application.
///
/// Uses `clap` to parse commands and `inquire` to interactively prompt the user.
#[derive(Parser)]
#[command(name = "Simple Todo CLI", version)]
struct Cli {
    /// Subcommands to execute
    #[command(subcommand)]
    command: Commands,
}

/// Supported commands for the to-do CLI.
#[derive(Subcommand)]
enum Commands {
    /// Add a new task.
    ///
    /// Optionally accepts the task description as a command argument.
    Add {
        /// The task description to add. If not provided, the user will be prompted.
        #[arg()]
        task: Option<String>,
    },

    /// List all tasks with their status.
    List,

    /// Remove one or more tasks by selecting from a list.
    Remove,

    /// Mark one or more tasks as completed by selecting from a list.
    Complete,
}

/// Main entry point of the application.
///
/// Parses CLI arguments and runs the appropriate command.
///
/// # Errors
/// Returns an error if any IO or user prompt fails.
///
/// # Examples
/// ```no_run
/// // Run the application with arguments from the command line.
/// fn main() -> anyhow::Result<()> {
///     // ...
/// }
/// ```
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            // Get task either from argument or prompt user
            let task = match task {
                Some(t) => t,
                None => Text::new("What task would you like to add?").prompt()?,
            };

            // Read existing todos, assign next ID, add new task, then save
            let mut todos = read_todos()?;
            let next_id = todos.last().map(|t| t.id + 1).unwrap_or(1);
            todos.push(TodoItem {
                id: next_id,
                task,
                completed: false,
            });
            write_todos(&todos)?;
            println!("✅ Task added!");
        }

        Commands::List => {
            // Read and display all todos
            let todos = read_todos()?;
            if todos.is_empty() {
                println!("📭 No tasks found.");
            } else {
                for todo in &todos {
                    let status = if todo.completed { "✅" } else { "❌" };
                    println!("[{}] {} {}", todo.id, status, todo.task);
                }
            }
        }

        Commands::Remove => {
            // Read todos, prompt user to select tasks to remove, then save updated list
            let mut todos = read_todos()?;
            if todos.is_empty() {
                println!("📭 No tasks to remove.");
                return Ok(());
            }

            let options: Vec<String> = todos
                .iter()
                .map(|t| format!("[{}] {}", t.id, t.task))
                .collect();

            let selected = MultiSelect::new("Select tasks to remove:", options).prompt()?;
            let ids_to_remove: Vec<usize> = selected
                .iter()
                .map(|s| s.split(']').next().unwrap()[1..].parse::<usize>().unwrap())
                .collect();

            todos.retain(|t| !ids_to_remove.contains(&t.id));

            write_todos(&todos)?;
            println!("🗑️ Task removed.");
        }

        Commands::Complete => {
            // Read todos, prompt user to select incomplete tasks to mark completed, then save
            let mut todos = read_todos()?;
            let incomplete: Vec<_> = todos.iter().filter(|t| !t.completed).collect();

            if incomplete.is_empty() {
                println!("🎉 All tasks are already completed!");
                return Ok(());
            }

            let options: Vec<String> = incomplete
                .iter()
                .map(|t| format!("[{}] {}", t.id, t.task))
                .collect();

            let selected =
                MultiSelect::new("Select tasks to mark as completed:", options).prompt()?;
            let ids_to_complete: Vec<usize> = selected
                .iter()
                .map(|s| s.split(']').next().unwrap()[1..].parse::<usize>().unwrap())
                .collect();

            for todo in &mut todos {
                if ids_to_complete.contains(&todo.id) {
                    todo.completed = true;
                }
            }
            write_todos(&todos)?;
            println!("🎯 Selected tasks marked as completed!");
        }
    }

    Ok(())
}
