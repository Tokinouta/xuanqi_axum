pub mod model;
pub mod web_service;

use actix_web::{web, App, HttpServer};
use web_service::{echo, hello, index, manual_hello};

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
