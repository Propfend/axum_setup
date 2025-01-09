use crate::error::result::Result;

use axum::{routing::get, Router};

mod error;
mod http_routes;

const ADDRES: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(http_routes::home::home))
        .route("/health", get(http_routes::health::health))
        .route("/favicon.ico", get(http_routes::favicon::favicon))
        .route("/blog", get(http_routes::blog::blog_main));

    axum::Server::bind(&ADDRES.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
