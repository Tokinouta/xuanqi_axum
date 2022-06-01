use actix_web::{
    get, post,
    http::{
        header::{ContentType}, StatusCode,
    }, HttpRequest, HttpResponse, Responder, web,
};
use mongodb::Client;

use crate::model::{Repo, User};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

#[get("/ra")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    let _path = "static/index.html";
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../static/index.html"))
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
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/create_user")]
pub async fn create_user(client: web::Data<Client>, user: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(user.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
