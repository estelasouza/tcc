use actix_web::FromRequest;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum::extract::Path;  
use uuid::Uuid;
use crate::ports::inbound::users;
use crate::adpters::db_out::users::MongoRepo;


pub async fn health(Path(id):Path<String>) -> impl IntoResponse{
    let greeting = id.as_str();


    (StatusCode::OK, String::from(greeting))
}


pub async fn create_user(Json(payload): Json<users::CreateUser>) -> impl IntoResponse {
    let user = users::User {
        id: Uuid::new_v4(),
        username: payload.username,
        age: payload.age
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn update_user(Json(payload): Json<users::CreateUser>) -> impl IntoResponse {
    let user = users::User {
        id: Uuid::new_v4(),
        username: payload.username,
        age: payload.age

    };

    (StatusCode::CREATED, Json(user))
}

pub async fn delete_user(id: String) -> impl IntoResponse {
    // fn to delete ( in mongo )
   
   
    StatusCode::NOT_FOUND
}