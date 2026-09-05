use chrono::prelude::*;
use serde_json::{from_reader, to_string_pretty};
use std::fs;
use std::io::{BufReader, Write};
use std::path::Path;

use crate::types::{Task, TaskList};

fn exit_with_error(context: &str, err: impl std::fmt::Display) -> ! {
    eprintln!("{context}: {err}");
    std::process::exit(1);
}

fn ensure_tasks_file() {
    let path = "tasks.json";
    if Path::new(path).exists() {
        return;
    }

    print!("Tasks file not found. Create it now? [y/N] ");
    std::io::stdout().flush().ok();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input. Aborting.");
        std::process::exit(1);
    }

    let answer = input.trim().to_lowercase();
    if answer == "y" || answer == "yes" {
        let empty = TaskList { tasks: vec![] };
        let json = to_string_pretty(&empty)
            .unwrap_or_else(|err| exit_with_error("Failed to serialize empty task list", err));
        if let Err(err) = fs::write(path, json) {
            exit_with_error("Failed to write tasks file", err);
        }
        println!("Created {path}");
    } else {
        eprintln!("Aborting: tasks.json is required to continue.");
        std::process::exit(1);
    }
}

pub fn now_string() -> String {
    let now = Local::now();
    now.to_string()
        .replacen(" ", "T", 1)
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .to_string()
}

pub fn read_tasks_file() -> Vec<Task> {
    ensure_tasks_file();
    let file = match fs::File::open("tasks.json") {
        Ok(file) => file,
        Err(err) => exit_with_error("Failed to open tasks file", err),
    };
    let reader = BufReader::new(file);
    let list: TaskList = match from_reader(reader) {
        Ok(list) => list,
        Err(err) => exit_with_error("Failed to parse tasks file", err),
    };
    list.tasks
}

pub fn write_tasks_file(tasks: Vec<Task>) {
    let list = TaskList { tasks };
    let json = match to_string_pretty(&list) {
        Ok(json) => json,
        Err(err) => exit_with_error("Failed to serialize tasks", err),
    };
    if let Err(err) = fs::write("tasks.json", json) {
        exit_with_error("Failed to write tasks file", err);
    }
}

pub fn load_task(id: u128) -> (Vec<Task>, usize) {
    let tasks = read_tasks_file();
    let pos = tasks
        .iter()
        .position(|task| task.id == id)
        .unwrap_or_else(|| {
            eprintln!("Task {id} not found.");
            std::process::exit(1);
        });
    (tasks, pos)
}
