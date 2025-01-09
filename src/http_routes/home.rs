use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub async fn home() -> impl IntoResponse {
    HelloTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

impl IntoResponse for HelloTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
