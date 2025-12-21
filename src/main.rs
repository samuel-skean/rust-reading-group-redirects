mod frontend;

use axum::{Router, response::Redirect, routing::get};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[axum::debug_handler]
async fn redirect_handler() -> Redirect {
    Redirect::temporary("http://example.com")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/redirect", get(redirect_handler))
        .route("/temp-dynamic", get(frontend::temp_dynamic_handler))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to initialize application");
}
