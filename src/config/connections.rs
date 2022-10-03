use std::net::SocketAddr;
use axum::{
    Router,
    
};

// #[tokio::main]
// pub fn http_server(app: Router)  {
    
// }


use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult,UpdateResult, DeleteResult},
    
};
use crate::ports::outbound::book::Book;

pub trait MongoBD {
    fn init()-> Self ;
    fn create(&self,new_user:Book) -> Result<InsertOneResult, Error> ;
    fn update(&self,id:String, new_user: &Book)->Result<UpdateResult, Error>;
    fn delete(&self,id: String) ->Result<DeleteResult, Error>;
    fn get(&self,id:String)-> Result<Book, Error>;

}
