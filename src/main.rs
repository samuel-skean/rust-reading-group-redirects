use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {"I have big brain"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Failed to initialize application");
}