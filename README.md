# 📝 Todo CLI

A simple and interactive command-line tool written in Rust to manage todos in your project repositories.  
Tasks are stored in a local `todo.json` file at the root of the repository.

## ✨ Features

- Add tasks via prompt or directly from the command line.
- List all tasks with completion status.
- Remove one or multiple tasks interactively.
- Mark tasks as completed (individually or in bulk).
- JSON-based persistence (portable and versionable with Git).

## 🚀 Getting Started

Install with Cargo:

```bash
cargo install simple-todo-cli
```

## 🛠 Usage

```bash
todo add [TASK]        # Add a new task (interactive if not provided)
todo list              # Show all tasks
todo remove            # Remove tasks (interactive multi-select)
todo complete          # Mark tasks as completed (interactive multi-select)
```

Examples:

```bash
todo add "Write README"
todo complete
todo remove
```

## 📂 Storage

Todos are saved in a `todo.json` file in the current working directory. You can commit this file with your project if you want to share task status.

## 📄 License

MIT License. See `LICENSE` for more information.
