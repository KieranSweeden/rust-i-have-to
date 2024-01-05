use axum::{response::Html, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(get_todos))
}

async fn get_todos() -> Html<&'static str> {
    Html("Hello from the todo get route!")
}
