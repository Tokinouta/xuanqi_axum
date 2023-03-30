use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use redis::{Client, Commands};

async fn save_text(State(client): State<Client>, Json(req): Json<String>) -> impl IntoResponse {
    let mut con = client.get_connection().unwrap();

    let key = "name";
    // let value = req.as_str();
    con.set::<&str, String, ()>(key, req).unwrap();
    (StatusCode::OK, "Saved Successfully")
}

fn save_file() {}

pub fn router() -> Router {
    Router::new().route("/save-txt", post(save_text))
}
