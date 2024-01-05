use axum::{response::Html, routing::get, Router};
use tower_livereload::LiveReloadLayer;

mod todo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/todos", todo::routes())
        .route(
            "/",
            get(|| async { Html("Hello from the home get route!") }),
        )
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
