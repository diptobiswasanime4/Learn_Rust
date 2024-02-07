use rocket::{
    get, post, put, patch, delete,
    http::Status,
    response::status::Custom,
    serde::json::Json,
    State,
};

use crate::{
    model::{AppState, Todo, UpdateTodo},
    response::{GenericResponse, TodoListResponse}
};

mod model;
mod response;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            health_checker_handler,
            todos_list_handler,
            create_todo_handler,
        ],
    )
}

#[get("/health")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Health is good.";

    let resp_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    Ok(Json(resp_json))
}


#[get("/todos")]
pub async fn todos_list_handler(
    data: &State<AppState>    
) -> Result<Json<TodoListResponse>, Status> {
    let v = data.todo_db.lock().unwrap();

    let todos: Vec<Todo> = v.iter().filter_map(|t| Some(t.clone())).collect();

    let resp_json = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Ok(Json(resp_json))
}


#[get("/create")]
pub async fn create_todo_handler() -> &'static str {
    "Create Todo Page."
}