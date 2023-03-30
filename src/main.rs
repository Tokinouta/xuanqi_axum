pub mod entities;
pub mod database;
pub mod web_service;
pub mod middleware;
pub mod states;

use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};

use crate::{states::AppState, web_service::hello, middleware::my_middleware};


#[tokio::main]
async fn main() {
    let state = AppState::new().await;
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
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), my_middleware))
        .with_state(state);
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
