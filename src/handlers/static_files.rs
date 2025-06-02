use super::*;
use axum::response::IntoResponse;
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn routes() -> Router<AppStateRef> {
    Router::new().nest_service("/account/images", ServeDir::new("html/images"))
} 