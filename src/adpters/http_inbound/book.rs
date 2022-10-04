use crate::config::connections::MongoBD;
use axum::extract::{Extension, Path};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::ports::inbound::book::{CreateBook, transform_inbound_to_domain};
use crate::ports::outbound::book::transform_domain_to_outbound;
use crate::domain::bussines_logical::book as book_domain;

use crate::adpters::db_outbound::book::MongoRepo;
use std::sync::Arc;

pub async fn get_by_name(
    Path(id): Path<String>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    let result = state.get(id.to_string());
    match result.unwrap() {
        value => (StatusCode::OK, Json(value))
    };
   
}

pub async fn create(
    Json(payload): Json<CreateBook>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    let book = transform_inbound_to_domain(payload);
    let book_out = transform_domain_to_outbound(book);
    let final_status = state.create(book_out);
    (StatusCode::CREATED, Json(final_status.unwrap()))
}

pub async fn update(
    Path(name): Path<String>,
    Json(payload): Json<CreateBook>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {

    let book =  transform_inbound_to_domain(payload) ;
    let book_out = transform_domain_to_outbound(book);
    let final_status = state.update(name,&book_out);
    (StatusCode::OK, Json(final_status.unwrap()))
}

pub async fn delete(
    Path(name): Path<String>,
    Extension(state): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    
    let delete = state.delete(name);
    (StatusCode::OK,Json(delete.unwrap()))
}