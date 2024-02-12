use crate::{
    model::{AppState, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse}
};
use rocket::{
    get, post, put, patch, delete,
    http::Status,
    response::status::Custom,
    serde::json::Json,
    State
};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod model;
mod response;

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();

    rocket::build()
        .manage(app_data)
        .mount(
        "/",
        routes![
            health_checker_handler,
            create_todo_handler,
            list_todos_handler,
            get_todo_handler,
            edit_todo_handler,
            delete_todo_handler
        ],
    )
}

#[get("/health")]
async fn health_checker_handler() -> &'static str {
    "Good Health."
}

#[get("/todos")]
async fn create_todo_handler (
    data: &State<AppState>
) -> Result<Json<TodoListResponse>, Status> {
    let vec = data.todo_db.lock().unwrap();

    let todos: Vec<Todo> = vec.clone().into_iter().collect();

    let json_resp = TodoListResponse {
        status: String::from("Success"),
        results: todos.len(),
        todos,
    };
    Ok(Json(json_resp))
}

#[post("/todos", data = "<body>")]
async fn list_todos_handler (
    mut body: Json<Todo>,
    data: &State<AppState>
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.title == body.title {
            let err_resp = GenericResponse {
                status: String::from("fail"),
                message: format!("Todo with title: '{}' already exists", todo.title)
            };
            return Err(Custom(Status::Conflict, Json(err_resp)))
        }
    }

    let uuid_id = Uuid::new_v4();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_resp = SingleTodoResponse {
        status: String::from("Success"),
        data: TodoData {
            todo: todo.into_inner(),
        }
    };

    Ok(Json(json_resp))
}

#[get("/todos/<id>")]
async fn get_todo_handler (
    id: String,
    data: &State<AppState>
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.id == Some(id.to_owned()) {
            let json_resp = SingleTodoResponse {
                status: String::from("Success"),
                data: TodoData {todo: todo.clone()}
            };
            return Ok(Json(json_resp))
        }
    }

    let err_resp = GenericResponse {
        status: String::from("Failure"),
        message: format!("Todo with id: {} doesn't exist", id)
    };
    Err(Custom(Status::NotFound, Json(err_resp)))
}

#[put("/todos/<id>", data = "<body>")]
async fn edit_todo_handler (
    id: String,
    body: Json<UpdateTodoSchema>,
    data: &State<AppState>
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            let title = body.title.to_owned().unwrap_or(todo.title.to_owned());
            let payload = Todo {
                id: todo.id.to_owned(),
                title: if !title.is_empty() {
                    title
                } else {
                    todo.title.to_owned()
                },
                completed: if body.completed.is_some() {
                    body.completed
                } else {
                    todo.completed
                }
            };
            *todo = payload;

            let json_resp = SingleTodoResponse {
                status: String::from("Success"),
                data: TodoData { todo: todo.clone() }
            };
            return Ok(Json(json_resp))
        }
    }

    let err_resp = GenericResponse {
        status: String::from("Failure"),
        message: format!("Todo with id: {} not found", id)
    };

    Err(Custom(Status::NotFound, Json(err_resp)))
}
#[delete("/todos/<id>")]
async fn delete_todo_handler (
    id: String,
    data: &State<AppState>
) -> Result <Status, Custom<Json<GenericResponse>>> {

    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            vec.retain(|t| t.id != Some(id.to_owned()));
            return Ok(Status::NoContent)
        }
    }

    let 


    
}