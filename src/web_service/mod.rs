use std::collections::HashMap;

// use actix_session::Session;
// use actix_web::{
//     get,
//     http::{header::ContentType, StatusCode},
//     post, web, HttpRequest, HttpResponse, Responder,
// };
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension, Form, Json, Router,
};
use mongodb::Client;

use crate::model::{
    database::{DB_NAME, USERS_COLL},
    Repo, User,
};

use askama::Template;

mod authentication;
pub use authentication::*;

// #[get("/ra")]
// pub async fn index(session: Session, _req: HttpRequest) -> impl Responder {
//     let _path = "static/index.html";
//     // RequestSession trait is used for session access
//     let mut counter = 1;
//     if let Some(count) = session.get::<i32>("counter").unwrap() {
//         // log::info!("SESSION value: {}", count);
//         counter = count + 1;
//         session.insert("counter", counter).unwrap();
//         HttpResponse::build(StatusCode::OK)
//             .content_type(ContentType::plaintext())
//             .body(include_str!("../static/index.html"))
//     } else {
//         session.insert("counter", counter).unwrap();
//         HttpResponse::build(StatusCode::OK).body("new")
//     }
// }

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    is_Logged_In: bool,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

pub async fn hello() -> impl IntoResponse {
    HtmlTemplate(HelloTemplate {
        is_Logged_In: false,
    })
}

pub async fn hello_html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// #[post("/echo")]
pub async fn echo(Query(q): Query<HashMap<String, String>>, req_body: String) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Query: {:?}, Body: {}", q, req_body),
    )
        .into_response()
}

// #[get("/create_repo")]
pub async fn create_repo(
    Extension(client): Extension<Client>,
    Form(form): Form<Repo>,
) -> impl IntoResponse {
    let collection = client.database(DB_NAME).collection(USERS_COLL);
    let result = collection.insert_one(form, None).await;
    match result {
        Ok(_) => (StatusCode::OK, "user added").into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

// #[post("/create_user")]
// pub async fn create_user(client: web::Data<Client>, user: web::Form<User>) -> HttpResponse {
//     let collection = client.database(DB_NAME).collection(USERS_COLL);
//     let result = collection.insert_one(user.into_inner(), None).await;
//     match result {
//         Ok(_) => HttpResponse::Ok().body("user added"),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
