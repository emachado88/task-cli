# Task Tracker CLI

A simple command-line interface (CLI) to track and manage tasks, written in Rust. Tasks are stored in a local JSON file (`tasks.json`) in the current working directory.

This project is an implementation of the [Task Tracker](https://roadmap.sh/projects/task-tracker) project from [roadmap.sh](https://roadmap.sh).

---

## Features

- **Add tasks**: Create new tasks with descriptions.
- **Update tasks**: Edit existing task descriptions by ID.
- **Delete tasks**: Remove tasks by ID.
- **Track status**: Mark tasks as `todo`, `in-progress`, or `done`.
- **List tasks**: View all tasks or filter by status (`todo`, `in-progress`, `done`).
- **File persistence**: Automatically prompts to create `tasks.json` if it doesn't already exist.

---

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Rust 2024 edition supported)

---

## Build

Clone the repository and build the binary:

```bash
git clone https://github.com/<your-username>/task-cli.git
cd task-cli
cargo build --release
```

The compiled binary will be placed at:

```bash
./target/release/task-cli
```

_(Optional)_ Copy or install the binary into your `PATH` (e.g. `~/.local/bin` or `/usr/local/bin`) to run `task-cli` from any directory:

```bash
cargo install --path .
```

---

## Usage

Once in the compiled binary's directory, run the CLI with the following commands:

```bash
./task-cli <command>
```

> **Note:** If `tasks.json` does not exist in the working directory when executing a command, the CLI will ask whether to create it (`[y/N]`).

### 1. Add a Task

```bash
./task-cli add "Buy groceries"
```

### 2. List Tasks

List all tasks:

```bash
./task-cli list
```

Filter tasks by status:

```bash
./task-cli list todo
./task-cli list in-progress
./task-cli list done
```

### 3. Update a Task

Update a task's description by its ID:

```bash
./task-cli update 1 "Buy groceries and cook dinner"
```

### 4. Mark Task Status

Mark a task as in progress:

```bash
./task-cli mark-in-progress 1
```

Mark a task as done:

```bash
./task-cli mark-done 1
```

### 5. Delete a Task

Delete a task by its ID:

```bash
./task-cli delete 1
```

### 6. Help

To see all available commands and flags:

```bash
./task-cli --help
```

Or get help for a specific subcommand:

```bash
./task-cli add --help
./task-cli list --help
```

---

## Data Model

Tasks are stored in `tasks.json` in the following format:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Buy groceries",
      "status": "todo",
      "created_at": "2026-09-06T10:00:00.000000000",
      "updated_at": "2026-09-06T10:00:00.000000000"
    }
  ]
}
```
