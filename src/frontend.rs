use axum::{http::StatusCode, response::Html};
use minijinja::{Environment, context, path_loader};

fn create_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
}
#[axum::debug_handler]
pub async fn temp_dynamic_handler() -> (StatusCode, Html<String>) {
    let env = create_environment();
    let Ok(template) = env.get_template("temp-dynamic.html.minijinja") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error getting template")),
        );
    };
    let Ok(rendered) = template.render(context!(name => "Jimmy")) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error rendering template")),
        );
    };
    (StatusCode::OK, Html(rendered))
}
