use axum::{extract::Query, http::StatusCode, response::Html};
use minijinja::{Environment, context, path_loader};

fn create_environment() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
}

#[derive(serde::Deserialize)]
pub struct NameInfo {
    name: String,
}

#[axum::debug_handler]
pub async fn temp_dynamic_handler(name_info: Query<NameInfo>) -> (StatusCode, Html<String>) {
    let env = create_environment();
    let Ok(template) = env.get_template("temp-dynamic.html.jinja2") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error getting template")),
        );
    };
    let Ok(rendered) = template.render(context!(name => name_info.name)) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error rendering template")),
        );
    };
    (StatusCode::OK, Html(rendered))
}
