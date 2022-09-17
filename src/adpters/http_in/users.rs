use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::ports::inbound::users;

pub async fn health() -> &'static str {
    print!("aq");
    "Hello, World!"
}


pub async fn create_user(Json(payload): Json<users::CreateUser>) -> impl IntoResponse {
    let user = users::User {
        id: 1333,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
