use crate::storage::{load_task, now_string, read_tasks_file, write_tasks_file};
use crate::types::{Status, Task};

pub fn add_task(description: String) {
    let mut tasks = read_tasks_file();
    let id = tasks.len() as u128 + 1;
    let now = now_string();
    let task = Task::new(id, description, Status::Todo, now.clone(), now);
    let msg = format!(
        "Task added: #{} [{}] {}",
        task.id, task.status, task.description
    );
    tasks.push(task);
    write_tasks_file(tasks);
    println!("{msg}");
}

pub fn delete_task(id: u128) {
    let (mut tasks, pos) = load_task(id);
    let task = tasks.remove(pos);
    write_tasks_file(tasks);
    println!("Deleted task {}: {}", task.id, task.description);
}

pub fn update_task(id: u128, description: String) {
    let (mut tasks, pos) = load_task(id);
    let task = &mut tasks[pos];
    task.description = description;
    task.updated_at = now_string();
    let msg = format!(
        "Task updated: #{} [{}] {}",
        task.id, task.status, task.description
    );
    write_tasks_file(tasks);
    println!("{msg}");
}

pub fn update_status(id: u128, status: Status) {
    let (mut tasks, pos) = load_task(id);
    let task = &mut tasks[pos];
    task.status = status;
    task.updated_at = now_string();
    let msg = format!(
        "Task updated: #{} [{}] {}",
        task.id, task.status, task.description
    );
    write_tasks_file(tasks);
    println!("{msg}");
}

pub fn list_tasks(filter: Option<Status>) {
    let tasks = read_tasks_file();
    let filtered: Vec<&Task> = tasks
        .iter()
        .filter(|task| filter.map_or(true, |status| task.status == status))
        .collect();
    if filtered.is_empty() {
        println!(
            "No{}tasks found",
            filter.map_or(" ".to_string(), |status| format!(" {} ", status.as_str()))
        );
        return;
    }

    println!("Tasks:");
    for task in filtered {
        println!("#{} [{}]  {}", task.id, task.status, task.description);
    }
}
