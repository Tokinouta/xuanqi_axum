pub mod model;
pub mod web_service;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{cookie::Key, web, App, HttpServer};
use web_service::{create_user, echo, hello, index, login, logout, manual_hello, signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = model::database::create_client().await;
    model::database::list_database_names(&client).await;

    // The secret key would usually be read from a configuration file/environment variables.
    let secret_key = Key::generate();
    let redis_connection_string = "redis://127.0.0.1:6379";
    let store = RedisSessionStore::new(redis_connection_string).await.unwrap();

    let endpoint = format!("0.0.0.0:{}", 8080);
    HttpServer::new(move || {
        // create cookie identity backend (inside closure, since policy is not Clone)
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .wrap(IdentityService::new(policy))
            // Add session management to your application using Redis for session state storage
            .wrap(SessionMiddleware::new(
                store.clone(),
                secret_key.clone(),
            )) // wrap policy into middleware identity middleware
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(index)
            .service(create_user)
            .service(signup)
            .service(login)
            .service(logout)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(endpoint)?
    .run()
    .await
}
