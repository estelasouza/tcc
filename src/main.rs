use crate::adpters::db_outbound::book::MongoRepo;
use crate::adpters::http_inbound::book;
use crate::config::connections::Http;
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use config::connections::MongoBD;
use std::sync::Arc;
pub mod adpters;
pub mod config;
pub mod domain;
pub mod ports;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let state = Arc::new(MongoRepo::init());

    // build our application with a single route
    let app = Router::new()
        .route("/:name", get(book::get_by_id))
        .route("/users", post(book::create))
        .route("/users/:name", put(book::update))
        .route("/:name", delete(book::delete))
        .layer(Extension(Arc::clone(&state)));

    Http::new(app);

    Ok(())
}
