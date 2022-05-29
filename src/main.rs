pub mod model;
pub mod web_service;

use actix_web::{web, App, HttpServer};
use web_service::{echo, hello, index, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = model::database::create_client().await;
    model::database::list_database_names(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
