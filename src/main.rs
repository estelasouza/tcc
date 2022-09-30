use axum::{
    routing::{get, post, put,delete},
    Router,
    extract::Extension
    
};
use config::MongoBD;
use std::net::SocketAddr;
pub mod adpters;
pub mod ports;
pub mod config;
pub mod domain;
use crate::adpters::http_inbound::users;
use std::sync::Arc;
use crate::adpters::db_outbound::users::MongoRepo;
use crate::ports::outbound::users::User;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), ()>  {

    let state = Arc::new(MongoRepo::init());


   // build our application with a single route
    let app = Router::new()
        .route("/:name", get(users::get_by_id))
        .route("/users", post(users::create_user))
        .route("/users/:name", put(users::update_user))
        .route("/:name", delete(users::delete_user))
        .layer(Extension(Arc::clone(&state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

        Ok(())
      
}


