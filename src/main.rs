pub mod model;
pub mod web_service;

// use actix_cors::Cors;
// use actix_identity::{CookieIdentityPolicy, IdentityService};
// use actix_session::{SessionMiddleware, storage::RedisSessionStore};
// use actix_web::{cookie::Key, web, App, HttpServer};
use axum::{
    http::Method,
    routing::get,
    Router, Extension,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use crate::web_service::{hello, hello_html, echo};


#[tokio::main]
async fn main() {
    let client = model::database::create_client().await;
    // build our application with a route
    let app = Router::new()
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        )
        .route("/", get(hello))
        .route("/hello_html", get(hello_html))
        .route("/echo", get(echo))
        .layer(Extension(client));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // model::database::list_database_names(&client).await;

    // The secret key would usually be read from a configuration file/environment variables.
    // let secret_key = Key::generate();
    // let redis_connection_string = "redis://127.0.0.1:6379";
    // let store = RedisSessionStore::new(redis_connection_string).await.unwrap();
}
