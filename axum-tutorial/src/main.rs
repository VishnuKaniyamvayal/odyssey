use core::str;
use tower_http::compression::CompressionLayer;


use axum::extract::FromRef;
use axum::{Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::error::ApiError;
use crate::response::{ApiResponse, Message};
use crate::{error::my_function};
use crate::rounters::init_user_router;

mod response;
mod error;
mod rounters;
mod db;

fn init_router(state: AppState) -> Router {
    let user_router = init_user_router();
    Router::new()
        .route("/", get(get_user))
        .layer(CompressionLayer::new())
        .route("/posttest", post(post_handler))
        .route("/api/v1", get(my_function))
        .merge(user_router)
        .with_state(state)
}
// test change
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    api_state: ApiState
}

#[derive(Clone)]
pub struct ApiState {}

impl FromRef<AppState> for ApiState {
    fn from_ref(app_state: &AppState) -> ApiState {
        app_state.api_state.clone()
    }
}

async fn get_user() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::JsonData(vec![Message { message: "hello".to_string() }]))
}

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Deserialize)]
struct Submission {
    message: String,
}

use axum_extra::{TypedHeader, headers::Origin};


async fn post_handler(
    TypedHeader(origin): TypedHeader<Origin>
) -> Result<ApiResponse, ApiError> {
    let user = User { name: "Vishnu".into(), age: 25 };
    let json = serde_json::to_string(&user).unwrap();
    Ok(ApiResponse::JsonData(vec![Message { message: origin.to_string() }, Message { message: json }]))
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str).await
        .expect("can't connect to database");

    let app_state = AppState { db: pool, api_state: ApiState {} };

    let router: Router = init_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
