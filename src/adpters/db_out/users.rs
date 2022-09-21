use crate::config::MongoBD;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult,UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::ports::out::users::User;
use std::env;
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoBD for MongoRepo {
    fn init() -> Self {
        
        let uri = match env::var("mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            age: new_user.age
           
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    fn update_user(&self, id: &'static str, new_user: User) ->  Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id};
        let doc = doc! {
            "$set":
            {
                "id": new_user.id,
                "name": new_user.name,            },
        };

        let update_doc = self.col.update_one(filter, doc, None).ok().expect("error to update");

        Ok(update_doc)
        
    }

    fn delete_user(&self, id:  &'static str) ->  Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter= doc! {"_id":obj_id};

        let user_detail = self.col.delete_one(filter, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }

    fn get_user(&self, id:  &String) ->  Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {
            "_id": obj_id
        };

        let user_doc = self.col.find_one(filter, None).ok().expect("error getting user");

        Ok(user_doc.unwrap())
    }
}