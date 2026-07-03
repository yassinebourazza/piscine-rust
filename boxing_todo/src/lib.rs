pub mod err;

pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let parsed = json::parse(&content)
            .map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        let title = parsed["title"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tasks = &parsed["tasks"];

        if tasks.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks_vec = Vec::new();

        for task in tasks.members() {
            tasks_vec.push(Task {
                id: task["id"].as_u32().unwrap_or(0),
                description: task["description"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                level: task["level"].as_u32().unwrap_or(0),
            });
        }

        Ok(TodoList {
            title,
            tasks: tasks_vec,
        })
    }
}