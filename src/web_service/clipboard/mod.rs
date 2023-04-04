use crate::states::AppState;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use redis::{Client, Commands};
use tokio::io::AsyncWriteExt;

async fn save_text(State(client): State<Client>, Json(req): Json<String>) -> impl IntoResponse {
    let mut con = client.get_connection().unwrap();

    let key = "name";
    // let value = req.as_str();
    con.set::<&str, String, ()>(key, req).unwrap();
    (StatusCode::OK, "Saved Successfully")
}

async fn save_file(mut multipart: Multipart) -> impl IntoResponse {
    // TODO: need to add user-wise storage
    let results = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        let filename = field.file_name().unwrap();
        results.push(format!("Receiving file {} with name {}", filename, name));

        // save the file to disk
        tokio::fs::File::create(filename)
            .await
            .unwrap()
            .into_std()
            .await
            .write_all(&field.bytes().await.unwrap())
            .unwrap();
    }
    (StatusCode::OK, results.join(" "))
}

async fn get_file() -> impl IntoResponse {
    (StatusCode::OK)
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/save-txt", post(save_text))
        .route("/save-file", put(save_file))
        .route("/get-file", get(get_file))
}
