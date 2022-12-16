pub mod entities;
pub mod model;
pub mod web_service;

// use actix_cors::Cors;
// use actix_identity::{CookieIdentityPolicy, IdentityService};
// use actix_session::{SessionMiddleware, storage::RedisSessionStore};
// use actix_web::{cookie::Key, web, App, HttpServer};
use axum::{extract::FromRef, routing::get, Extension, Router};
use axum_database_sessions::{
    AxumPgPool, AxumRedisPool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use axum_sessions_auth::{AuthSession, AuthSessionLayer, Authentication, AxumAuthConfig};
use lazy_static::lazy_static;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

use crate::{entities::User, web_service::hello};

#[derive(Clone)]
struct AppState {
    client: Client,
    database: Pool<Postgres>,
}

#[derive(Clone)]
struct HttpClient {}

impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.client.clone()
    }
}

#[derive(Clone)]
struct Database {}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

#[tokio::main]
async fn main() {
    // let client = model::database::create_client().await;
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    // let poll = connect_to_database().await.unwrap();

    // Build our application with some routes
    // let app = Router::new()
    // .route("/greet/:name", get(greet))

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    // let session_config = AxumSessionConfig::default().with_table_name("test_table");
    // let auth_config = AxumAuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    // let session_store =
    //     AxumSessionStore::<AxumPgPool>::new(Some(pool.clone().into()), session_config);

    let app = Router::new()
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new().allow_origin(Any).allow_methods(Any),
        )
        .route("/", get(hello))
        .with_state(AppState {
            client,
            database: pool,
        });
    // .merge(web_service::authentication::router())
    // .layer(AxumSessionLayer::new(session_store))
    // .layer(
    //     AuthSessionLayer::<UserInSession, i64, AxumRedisPool, Client>::new(Some(client)).with_config(auth_config),
    // );
    // .route("/hello_html", get(hello_html))
    // .route("/echo", get(echo))
    // .layer(Extension(client));

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
