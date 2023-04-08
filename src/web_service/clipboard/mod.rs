use std::io::Write;

use crate::states::AppState;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use hyper::{header, HeaderMap};
// use hyper::
use redis::{Client, Commands};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn save_text(State(client): State<Client>, Json(req): Json<String>) -> impl IntoResponse {
    let mut con = client.get_connection().unwrap();

    let key = "name";
    // let value = req.as_str();
    con.set::<&str, String, ()>(key, req).unwrap();
    (StatusCode::OK, "Saved Successfully")
}

async fn save_file(mut multipart: Multipart) -> impl IntoResponse {
    // TODO: need to add user-wise storage
    let mut results = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        let filename = field.file_name().unwrap();
        results.push(format!("Receiving file {} with name {}", filename, name));

        let file = tokio::fs::File::create(filename).await.unwrap();
        if let Ok(bytes) = field.bytes().await {
            // save the file to disk
            file.write_all(&bytes.iter().collect::<[u8]>()).await
        } else {
            return (StatusCode::OK, results.join(" "));
        }
        .unwrap();
        // .into_std()
        // .await
        // .write_all(&field.bytes().await.unwrap())
        // .unwrap();
    }
    (StatusCode::OK, results.join(" "))
}

async fn get_file(path: Option<Path<String>>) -> impl IntoResponse {
    if let Some(path) = path {
        let path = path.0;
        // `File` implements `AsyncRead`
        let mut file = match tokio::fs::File::open(&path).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        let mut buffer = Vec::new();

        // read the whole file
        match file.read_to_end(&mut buffer).await {
            Ok(_) => (),
            Err(_) => return Err((StatusCode::NOT_FOUND, "rarara".to_string())),
        };

        // convert the `Stream` into an `axum::body::HttpBody`
        let body = String::from_utf8(buffer).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "text/toml; charset=utf-8".parse().unwrap(),
        ); // TODO need to get the correct type
        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", path)
                .parse()
                .unwrap(),
        ); // TODO need to get the correct type
        Ok((headers, body))
    } else {
        Err((StatusCode::NOT_FOUND, "rarara".to_string()))
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/save-txt", post(save_text))
        .route("/save-file", put(save_file))
        .route("/get-file", get(get_file))
}
