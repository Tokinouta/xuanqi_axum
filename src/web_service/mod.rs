use actix_session::Session;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpRequest, HttpResponse, Responder,
};
use mongodb::Client;

use crate::model::{Repo, User, database::{DB_NAME, USERS_COLL}};

mod authentication;
pub use authentication::*;

#[get("/ra")]
pub async fn index(session: Session, _req: HttpRequest) -> impl Responder {
    let _path = "static/index.html";
    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        // log::info!("SESSION value: {}", count);
        counter = count + 1;
        session.insert("counter", counter).unwrap();
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body(include_str!("../static/index.html"))
    } else {
        session.insert("counter", counter).unwrap();
        HttpResponse::build(StatusCode::OK).body("new")
    }
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/create_repo")]
pub async fn create_repo(client: web::Data<Client>, form: web::Form<Repo>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(USERS_COLL);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/create_user")]
pub async fn create_user(client: web::Data<Client>, user: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(USERS_COLL);
    let result = collection.insert_one(user.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
