use axum::response::{Html, IntoResponse};

pub async fn blog_main() -> impl IntoResponse {
    Html("Blog")
}
