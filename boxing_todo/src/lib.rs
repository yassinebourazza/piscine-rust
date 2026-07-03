mod err;

use std::error::Error;
use std::fs;
use err::{ParseErr, ReadErr};

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



#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(e) => Some(&**e),
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub err: Box<dyn Error>,
}

impl fmt::Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}


impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {

        let mut tasksVec = Vec::new();

        let content = fs::read_to_string(path)
            .map_err(|e| ReadErr { err: Box::new(e) })?;


        let parsed = json::parse(&content)
            .map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        let title = parsed["title"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tasks = &parsed["tasks"];

        if !tasks.is_array() {
            return Err(Box::new(ParseErr::Malformed(
                "tasks is not array".into(),
            )));
        }

        if tasks.len() < 1 {
            return Err(Box::new(ParseErr::Empty));
        }


        for t in tasks.members() {
            tasksVec.push(Task {
                id: t["id"].as_u32().unwrap_or(0),
                description: t["description"].as_str().unwrap_or("").to_string(),
                level: t["level"].as_u32().unwrap_or(0),
            });
        }

        Ok(TodoList { title, tasks: tasksVec })
    }
}