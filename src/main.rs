mod todo;
use clap::{Parser, Subcommand};
use inquire::{MultiSelect, Text};
use todo::{TodoItem, read_todos, write_todos};

#[derive(Parser)]
#[command(name = "Todo", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg()]
        task: Option<String>,
    },
    List,
    Remove,
    Complete,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            let task = match task {
                Some(t) => t,
                None => Text::new("What task would you like to add?").prompt()?,
            };

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
