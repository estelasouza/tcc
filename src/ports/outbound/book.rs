use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::domain::bussines_logical::book::BookDomain;


#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub book_name: String, 
    pub description: String,
    pub is_test: bool,
}

pub fn transform_outbound(payload: BookDomain) -> Book{

    let new_doc = Book {
        id: None,
        book_name: payload.book_name,
        description: payload.description,
        is_test: payload.is_test
    };
    
    new_doc

}