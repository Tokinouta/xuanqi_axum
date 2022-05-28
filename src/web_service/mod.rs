use actix_web::{
    get, post,
    http::{
        header::{ContentType}, StatusCode,
    }, HttpRequest, HttpResponse, Responder,
};

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
