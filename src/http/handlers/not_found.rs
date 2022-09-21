use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
