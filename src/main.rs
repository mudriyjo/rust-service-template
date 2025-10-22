use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| hello_world()));
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn hello_world() -> String {
    "Hello, world!".to_string()
}
