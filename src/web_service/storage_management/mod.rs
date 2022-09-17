use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    let category_router = Router::new();
    let topic_router = Router::new();
    Router::new()
        .nest("/category", category_router)
        .nest("/topic", topic_router)
}
