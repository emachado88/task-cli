use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u128,
    pub description: String,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    pub fn new(
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
pub struct TaskList {
    pub tasks: Vec<Task>,
}
