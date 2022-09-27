
use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult,UpdateResult, DeleteResult},
    
};
use crate::ports::out::users::User;

pub trait MongoBD {
    fn init()-> Self ;
    fn create_user(&self,new_user:User) -> Result<InsertOneResult, Error> ;
    fn update_user(&self,id:String, new_user: &User)->Result<UpdateResult, Error>;
    fn delete_user(&self,id: String) ->Result<DeleteResult, Error>;
    fn get_user(&self,id:String)-> Result<User, Error>;

}
