use std::net::SocketAddr;
use hyper::server::conn::AddrIncoming;
use axum::Router;
use hyper::Server;
use axum::routing::IntoMakeService;
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

pub struct Http {
   pub http : Server<AddrIncoming,IntoMakeService<Router>>
}

impl  Http {
    pub fn new( app: Router) -> Self {
        let addr =  SocketAddr::from(([127, 0, 0, 1], 3000));

        let http = axum::Server::bind(&addr)
        .serve(app.into_make_service());
        
        Self { http }

    }

    pub async fn run(self) {
        self.http.await.unwrap()
    }
}