use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

pub enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response{
        match self {
            ApiResponse::OK => (StatusCode::OK).into_response(),
            ApiResponse::Created => (StatusCode::CREATED).into_response(),
            ApiResponse::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}