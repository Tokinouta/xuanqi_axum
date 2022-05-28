pub mod model;

use actix_files::{Files, NamedFile};
// use actix_session::{CookieSession, Session};
use actix_web::{
    error, get, post,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

#[get("/ra")]
async fn index(req: HttpRequest) -> impl Responder {
    let path = "static/index.html";
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("./static/index.html"))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    model::ra::test_mongodb().await;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}