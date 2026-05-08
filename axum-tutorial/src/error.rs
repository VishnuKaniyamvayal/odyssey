use axum::{response::Response ,http::{StatusCode}, response::IntoResponse};

pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError
}

use crate::response::ApiResponse;


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match  self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

pub async fn my_function() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK)
}