use std::collections::HashMap;

use askama::Template;
use axum::{response::{IntoResponse, Html}, extract::Query, http::StatusCode};

use super::HtmlTemplate;


#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    is_logged_in: bool,
}


pub async fn hello() -> impl IntoResponse {
    HtmlTemplate(HelloTemplate {
        is_logged_in: false,
    })
}

#[allow(dead_code)]
pub async fn hello_html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[allow(dead_code)]
pub async fn echo(Query(q): Query<HashMap<String, String>>, req_body: String) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Query: {:?}, Body: {}", q, req_body),
    )
        .into_response()
}
