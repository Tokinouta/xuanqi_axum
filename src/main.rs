pub mod model;
pub mod web_service;
pub mod entities;

// use actix_cors::Cors;
// use actix_identity::{CookieIdentityPolicy, IdentityService};
// use actix_session::{SessionMiddleware, storage::RedisSessionStore};
// use actix_web::{cookie::Key, web, App, HttpServer};
use axum::{
    routing::get,
    Router, Extension,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use axum_database_sessions::{AxumPgPool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore};
use axum_sessions_auth::{AuthSession, AuthSessionLayer, Authentication, AxumAuthConfig};

use crate::web_service::{hello};


#[tokio::main]
async fn main() {
    let client = model::database::create_client().await;
    // build our application with a

    // let poll = connect_to_database().await.unwrap();

    // let session_config = AxumSessionConfig::default()
    //     .with_table_name("test_table");
    // let auth_config = AxumAuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    // let session_store = AxumSessionStore::<AxumPgPool>::new(Some(poll.clone().into()), session_config);

    // Build our application with some routes
    // let app = Router::new()
        // .route("/greet/:name", get(greet))
        // .layer(AxumSessionLayer::new(session_store))
        // .layer(AuthSessionLayer::<User, i64, AxumPgPool, PgPool>::new(Some(poll)).with_config(auth_config));

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
                .allow_methods(Any),
        )
        .route("/", get(hello))
        // .route("/hello_html", get(hello_html))
        // .route("/echo", get(echo))
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
