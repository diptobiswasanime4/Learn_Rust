use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
}

pub struct AppState {
    pub todo_db: Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
