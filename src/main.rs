mod frontend;

use axum::{Router, response::Redirect, routing::get};
use rusqlite::{Connection, Result};
use tower_http::{services::ServeDir, trace::TraceLayer};

static LISTEN_ADDR: &str = "0.0.0.0:3000";

#[axum::debug_handler]
async fn redirect_handler() -> Redirect {
    let conn = Connection::open("test.db").expect("Failed to open database");
    let mut stmt = conn
        .prepare("SELECT URL FROM Redirect_Schedule WHERE Hour LIKE 23;")
        .unwrap();
    let mut link = stmt.query_row([], |row| Ok(row.get::<_, String>(0)?));

    Redirect::temporary(link.unwrap().as_str())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/redirect", get(redirect_handler))
        .route("/temp-dynamic", get(frontend::temp_dynamic_handler))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDR)
        .await
        .expect("Failed to bind");
    // TODO: Make this print a printable link.
    println!("Listening on {LISTEN_ADDR}.");
    axum::serve(listener, app)
        .await
        .expect("Failed to initialize application");
}
