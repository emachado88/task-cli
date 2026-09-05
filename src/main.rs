use chrono::prelude::*;
use clap::{Arg, Command, command};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string_pretty};
use std::fs;
use std::io::{self, BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => "todo",
            Status::InProgress => "in-progress",
            Status::Done => "done",
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Done => write!(f, "done"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u128,
    description: String,
    status: Status,
    created_at: String,
    updated_at: String,
}

impl Task {
    fn new(
        id: u128,
        description: String,
        status: Status,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            description,
            status,
            created_at,
            updated_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

fn now_string() -> String {
    let now = Local::now();
    now.to_string()
        .replacen(" ", "T", 1)
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .to_string()
}

fn ensure_tasks_file() {
    let path = "tasks.json";
    if Path::new(path).exists() {
        return;
    }

    print!("Tasks file not found. Create it now? [y/N] ");
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input. Aborting.");
        std::process::exit(1);
    }

    let answer = input.trim().to_lowercase();
    if answer == "y" || answer == "yes" {
        let empty = TaskList { tasks: vec![] };
        let json = to_string_pretty(&empty).expect("failed to serialize empty task list");
        fs::write(path, json).expect("failed to write tasks.json");
        println!("Created {path}");
    } else {
        eprintln!("Aborting: tasks.json is required to continue.");
        std::process::exit(1);
    }
}

fn read_tasks_file() -> Vec<Task> {
    ensure_tasks_file();
    let file = fs::File::open("tasks.json").expect("failed to open tasks.json");
    let reader = BufReader::new(file);
    let list: TaskList = from_reader(reader).expect("failed to parse tasks.json");
    list.tasks
}

fn write_tasks_file(tasks: Vec<Task>) {
    let list = TaskList { tasks };
    let json = to_string_pretty(&list).expect("failed to serialize tasks");
    fs::write("tasks.json", json).expect("failed to write tasks.json");
}

fn add_task(description: String) {
    let mut tasks = read_tasks_file();
    let id = tasks.len() as u128 + 1;
    let now = now_string();
    let task = Task::new(id, description, Status::Todo, now.clone(), now);
    tasks.push(task);
    write_tasks_file(tasks);
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("add")
                .about("adds a new task")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("description")
                        .help("task description to add")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("updates an existing task")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("delete")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("id")
                        .help("task id to delete")
                        .required(true)
                        .value_parser(clap::value_parser!(u128)),
                )
                .about("deletes a task"),
        )
        .subcommand(Command::new("mark-in-progress").about("marks a task as in progress"))
        .subcommand(Command::new("mark-done").about("marks a task as done"))
        .subcommand(Command::new("list").about("lists all tasks").subcommands([
            Command::new(Status::Todo.as_str()).about("lists all todo tasks"),
            Command::new(Status::InProgress.as_str()).about("lists all in progress tasks"),
            Command::new(Status::Done.as_str()).about("lists all done tasks"),
        ]))
        .get_matches();

    if matches.subcommand_matches("add").is_some() {
        let description = matches
            .get_one::<String>("description")
            .unwrap_or_else(|| {
                panic!("description is required");
            })
            .clone();
        add_task(description);
    }

    if let Some(list_matches) = matches.subcommand_matches("list") {
        let filter = match list_matches.subcommand_name() {
            Some(name) if name == Status::Todo.as_str() => Some(Status::Todo),
            Some(name) if name == Status::InProgress.as_str() => Some(Status::InProgress),
            Some(name) if name == Status::Done.as_str() => Some(Status::Done),
            _ => None,
        };
        let tasks = read_tasks_file();
        println!("Tasks:");
        let filtered: Vec<&Task> = tasks
            .iter()
            .filter(|task| filter.map_or(true, |status| task.status == status))
            .collect();
        if filtered.is_empty() {
            println!(
                "No{}tasks found",
                filter.map_or("".to_string(), |status| format!(" {} ", status.as_str()))
            );
            return;
        }
        for task in filtered {
            println!("#{} [{}] {}", task.id, task.status, task.description);
        }
    }
}
