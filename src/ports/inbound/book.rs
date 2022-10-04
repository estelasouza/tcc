use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::bussines_logical::book::BookDomain;
use crate::domain::bussines_logical::book as book_domain;

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


pub fn transform_inbound_to_domain(payload : CreateBook) -> BookDomain{
    let book = book_domain::BookDomain {
        id: None,
        book_name: payload.name,
        description: payload.description,
        is_test: false,
    };

    
    let result_book: book_domain::BookDomain = book_domain::validate_field(book) ;
    result_book
}