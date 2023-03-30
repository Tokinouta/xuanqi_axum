use axum::extract::FromRef;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

/// 程序的状态变量，这里主要是两个数据库连接
#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub database: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();

        // let poll = connect_to_database().await.unwrap();

        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

        // setup connection pool
        let database = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("can connect to database");

        AppState {
            client,
            database,
        }
    }
}

/// 把程序状态转为redis连接
impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.client.clone()
    }
}

/// 把程序状态转为postgres连接
impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}
