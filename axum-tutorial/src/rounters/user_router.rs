use axum::{Router, routing::get};

use crate::{error::ApiError, response::ApiResponse, AppState};
use std::sync::Arc;

async fn getUsers() -> Result<ApiResponse, ApiError> {
    return Ok(ApiResponse::OK);
}

async fn getUser() -> Result<ApiResponse, ApiError> {
    return Ok(ApiResponse::OK);
}

pub fn init_user_router() -> Router<AppState> {
   let router = Router::new()
    .route("/users", get(getUsers)).route("/user/{id}", get(getUser));

    router
}