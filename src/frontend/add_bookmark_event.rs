use axum::{http::StatusCode, response::Html};

use crate::frontend::*;

#[axum::debug_handler]
pub async fn add_bookmark_event() -> (StatusCode, Html<String>) {
    let env = RELOADER
        .acquire_env()
        .expect("Unable to acquire minijinja environment.");
    let Ok(template) = env.get_template("add-bookmark-event.html.jinja2") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error getting template")),
        );
    };
    let Ok(rendered) =
        template.render(context!(defaultStartDateTime => "1970-01-01 09:00", defaultEndDateTime => "1970-01-01 17:00", defaultTarget => "http://example.com"))
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error rendering template")),
        );
    };
    (StatusCode::OK, Html(rendered))
}
