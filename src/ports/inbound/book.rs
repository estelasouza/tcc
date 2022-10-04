use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateBook {
    pub name: String,
    pub description: String,
    pub is_test: bool,
}

#[derive(Serialize)]
pub struct Book {
    pub id: Option<Uuid>,
    pub book_name: String,
    pub description: String,
    pub is_test: bool,
}
