use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(get_todos))
}

async fn get_todos() -> &'static str {
    "Hello from the todo get route!"
}
