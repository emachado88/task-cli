mod storage;
mod tasks;
mod types;

use clap::{Arg, Command, command};

use tasks::{add_task, delete_task, list_tasks, update_status, update_task};
use types::Status;

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
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("updates an existing task")
                .arg_required_else_help(true)
                .args(
                    vec![
                        Arg::new("id")
                            .help("id of the task to update")
                            .required(true)
                            .value_parser(clap::value_parser!(u128)),
                        Arg::new("description")
                            .help("new description of the task")
                            .required(true)
                            .num_args(1)
                            .value_parser(clap::value_parser!(String)),
                    ]
                    .into_iter()
                    .collect::<Vec<Arg>>(),
                ),
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
        .subcommand(
            Command::new("mark-in-progress")
                .about("marks a task as in progress")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("id")
                        .help("task id to mark as in progress")
                        .required(true)
                        .value_parser(clap::value_parser!(u128)),
                ),
        )
        .subcommand(
            Command::new("mark-done")
                .about("marks a task as done")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("id")
                        .help("task id to mark as done")
                        .required(true)
                        .value_parser(clap::value_parser!(u128)),
                ),
        )
        .subcommand(Command::new("list").about("lists all tasks").subcommands([
            Command::new(Status::Todo.as_str()).about("lists all todo tasks"),
            Command::new(Status::InProgress.as_str()).about("lists all in progress tasks"),
            Command::new(Status::Done.as_str()).about("lists all done tasks"),
        ]))
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        let description = add_matches
            .get_one::<String>("description")
            .unwrap_or_else(|| {
                panic!("description is required");
            })
            .clone();
        add_task(description);
    }

    if let Some(delete_matches) = matches.subcommand_matches("delete") {
        let id = delete_matches.get_one::<u128>("id").unwrap_or_else(|| {
            panic!("id is required");
        });
        delete_task(id.to_owned());
    }

    if let Some(update_matches) = matches.subcommand_matches("update") {
        let id = update_matches.get_one::<u128>("id").unwrap_or_else(|| {
            panic!("id is required");
        });
        let description = update_matches
            .get_one::<String>("description")
            .unwrap_or_else(|| {
                panic!("description is required");
            });
        update_task(id.to_owned(), description.to_owned());
    }

    if let Some(update_matches) = matches.subcommand_matches("mark-in-progress") {
        let id = update_matches.get_one::<u128>("id").unwrap_or_else(|| {
            panic!("id is required");
        });
        update_status(id.to_owned(), Status::InProgress);
    }

    if let Some(update_matches) = matches.subcommand_matches("mark-done") {
        let id = update_matches.get_one::<u128>("id").unwrap_or_else(|| {
            panic!("id is required");
        });
        update_status(id.to_owned(), Status::Done);
    }

    if let Some(list_matches) = matches.subcommand_matches("list") {
        let filter = match list_matches.subcommand_name() {
            Some(name) if name == Status::Todo.as_str() => Some(Status::Todo),
            Some(name) if name == Status::InProgress.as_str() => Some(Status::InProgress),
            Some(name) if name == Status::Done.as_str() => Some(Status::Done),
            _ => None,
        };
        list_tasks(filter);
    }
}
