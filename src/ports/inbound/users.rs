use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub age: u32
}

#[derive(Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    pub age: u32,
}
