use super::*;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use std::path::PathBuf;

// Embed images into binary
const IMAGES: &[(&str, &[u8])] = &[
    ("EN.png", include_bytes!("../../html/images/EN.png")),
    ("VN.png", include_bytes!("../../html/images/VN.png")),
    ("logo.png", include_bytes!("../../html/images/logo.png")),
    ("background.jpg", include_bytes!("../../html/images/background.jpg")),
];

pub fn routes() -> Router<AppStateRef> {
    Router::new().route("/account/images/:filename", get(serve_image))
}

async fn serve_image(Path(filename): Path<String>) -> Response {
    // Find the requested image
    if let Some((_, image_data)) = IMAGES.iter().find(|(name, _)| *name == filename) {
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_static(match filename.split('.').last() {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            }),
        );

        (StatusCode::OK, headers, image_data.to_vec()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
} 