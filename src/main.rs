use axum::{
    routing::{get, post, put,delete},
    Router,
    extract::Extension
    
};
use config::connections::MongoBD;
pub mod adpters;
pub mod ports;
pub mod config;
pub mod domain;
use crate::adpters::http_inbound::book;
use std::sync::Arc;
use crate::adpters::db_outbound::book::MongoRepo;
use std::net::SocketAddr;


#[tokio::main]
async fn main() -> Result<(), ()>  {

    let state = Arc::new(MongoRepo::init());


   // build our application with a single route
    let app = Router::new()
        .route("/:name", get(book::get_by_id))
        .route("/users", post(book::create))
        .route("/users/:name", put(book::update))
        .route("/:name", delete(book::delete))
        .layer(Extension(Arc::clone(&state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

        Ok(())
      
}


