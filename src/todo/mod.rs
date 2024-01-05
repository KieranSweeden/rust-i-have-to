use askama::Template;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(get_todo))
}

#[derive(Template)]
#[template(path = "todo.html")]
struct HelloTemplate<'a> {
    todo_name: &'a str,
}

async fn get_todo() -> HelloTemplate<'static> {
    HelloTemplate {
        todo_name: "finish this project",
    }
}
