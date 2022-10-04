use crate::ports::outbound::book::Book;
use mongodb::{
    bson::extjson::de::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

pub trait MongoBD {
    fn init() -> Self;
    fn create(&self, new_user: Book) -> Result<InsertOneResult, Error>;
    fn update(&self, id: String, new_user: &Book) -> Result<UpdateResult, Error>;
    fn delete(&self, id: String) -> Result<DeleteResult, Error>;
    fn get(&self, id: String) -> Result<Book, Error>;
}

