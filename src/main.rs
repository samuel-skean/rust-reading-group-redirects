use axum::{Router, response::Redirect, routing::get};

#[axum::debug_handler]
async fn redirect_handler() -> Redirect {
    Redirect::temporary("http://example.com")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/redirect", get(redirect_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to initialize application");
}
