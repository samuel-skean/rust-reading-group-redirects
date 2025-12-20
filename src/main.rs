use axum::{Router, routing::get};

#[axum::debug_handler]
async fn root_handler() -> &'static str {
    "I have big brain"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to initialize application");
}
