use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
pub mod adpters;
pub mod ports;
pub mod config;
pub mod domain;

use crate::adpters::http_in::users;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(users::health))
        .route("/users", post(users::create_user))
        .route("/users", put(users::create_user))
        ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

