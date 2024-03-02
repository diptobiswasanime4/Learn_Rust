use rocket::{
    get, post, put, patch, delete
};

mod model;
mod response;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                health_checker,
                list_todos,
            ],
        )
}

#[get("/health")]
async fn health_checker() -> &'static str {
    "Good health."
}

#[get("/todos")]
async fn list_todos() -> &'static str {
    "List todos."
}