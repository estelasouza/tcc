use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    
};
use axum::extract::{Path,Extension};  
use crate::config::connections::MongoBD;

use crate::ports::inbound::book::{Book, CreateBook};
use crate::ports::outbound::book::Book as book_outbound;

use crate::adpters::db_outbound::book::MongoRepo;
use std::sync::Arc;

pub async fn get_by_id(    
Path(id):Path<String>,
Extension(state): Extension<Arc<MongoRepo>>,

) -> impl IntoResponse {
    

    let result = state.get(id.to_string());
    // aprender a colocar em um objeto -> deserializar -> 
    
    (StatusCode::OK, Json(result.unwrap()))
}


pub async fn create(
    Json(payload): Json<CreateBook>,
    Extension(state): Extension<Arc<MongoRepo>>,

) -> impl IntoResponse {
    let book = book_outbound {
        id: None,
        book_name: payload.name,
        description: payload.description,
        is_test: false
    };

    let final_status =  state.create(book);

    (StatusCode::CREATED, Json(final_status.unwrap()))
}

pub async fn update(
    Path(name):Path<String>,
    Json(payload): Json<CreateBook>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    let book = book_outbound {
        id: None,
        book_name: payload.name,
        description: payload.description,
        is_test: false
    };
    let final_status= state.update(name.to_string() , &book);


    (StatusCode::OK, Json(final_status.unwrap()))
}

pub async fn delete(
    Path(name):Path<String>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {

    state.delete( name);

    StatusCode::NOT_FOUND
}    

