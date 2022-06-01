pub mod model;
pub mod web_service;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use web_service::{create_user, echo, hello, index, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = model::database::create_client().await;
    model::database::list_database_names(&client).await;

    let endpoint = format!("0.0.0.0:{}", 8080);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(create_user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(endpoint)?
    .run()
    .await
}
