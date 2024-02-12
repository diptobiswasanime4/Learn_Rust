use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

pub struct AppState {
    pub todo_db: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(vec![
                Todo {
                    id: Some(String::from("a")),
                    title: String::from("Write JS."),
                    completed: Some(false),
                },
                Todo {
                    id: Some(String::from("b")),
                    title: String::from("Write TS."),
                    completed: Some(true),
                },
            ])),
        }
    }
}
