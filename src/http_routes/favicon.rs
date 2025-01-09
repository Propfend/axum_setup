use axum::response::{Html, IntoResponse};

pub async fn favicon() -> impl IntoResponse {
    Html("Blog")
}
