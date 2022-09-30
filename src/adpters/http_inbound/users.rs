use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    
};
use axum::extract::{Path,Extension};  
use uuid::Uuid;
use crate::config::MongoBD;
use crate::ports::inbound::users::CreateUser;
use crate::ports::outbound::users::User;

use crate::adpters::db_outbound::users::MongoRepo;
use std::sync::Arc;

pub async fn get_by_id(    
Path(id):Path<String>,
Extension(state): Extension<Arc<MongoRepo>>,

) -> impl IntoResponse {
    

    let result = state.get_user(id.to_string());
    // aprender a colocar em um objeto -> deserializar -> 
    
    (StatusCode::OK, Json(result.unwrap()))
}


pub async fn create_user(
    Json(payload): Json<CreateUser>,
    Extension(state): Extension<Arc<MongoRepo>>,

) -> impl IntoResponse {
    let user = User {
        id: None,
        name: payload.username,
        age: payload.age
    };

    let final_status =  state.create_user(user);

    (StatusCode::CREATED, Json(final_status.unwrap()))
}

pub async fn update_user(
    Path(name):Path<String>,
    Json(payload): Json<CreateUser>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        name: payload.username,
        age: payload.age

    };
    let final_status= state.update_user(name.to_string() , &user);


    (StatusCode::OK, Json(final_status.unwrap()))
}

pub async fn delete_user(
    Path(name):Path<String>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {

    state.delete_user( name);

    StatusCode::NOT_FOUND
}    

