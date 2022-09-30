use crate::config::MongoBD;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult,UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::ports::outbound::users::User;
use std::env;
use futures::stream::TryStreamExt;

const MONGO_URI: &str = "mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority";

pub struct MongoRepo {
    col: Collection<User>,
}


impl MongoBD for MongoRepo {
    fn init() -> Self {
        
        // let uri = match env::var(MONGO_URI) {
        //     Ok(v) => v.to_string(),
        //     Err(_) => format!("Error loading env variable"),
        // };
        let client = Client::with_uri_str("mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority").unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        print!("fui chamado {:?}", &col);
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

    fn update_user(&self, id: String, new_user: &User) ->  Result<UpdateResult, Error> {
        let obj_id = id;
        let filter = doc! { "name": obj_id};
        let doc = doc! {
            "$set":
            {
                "name": &new_user.name,            },
        };

        let update_doc = self.col.update_one(filter, doc, None).ok().expect("error to update");

        Ok(update_doc)
        
    }

    fn delete_user(&self, id:  String) ->  Result<DeleteResult, Error> {
        let obj_id = id ;
        let filter= doc! {"name":obj_id};

        let user_detail = self.col.delete_one(filter, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }

    fn get_user(&self, id:  String) ->  Result<User, Error> {
        let filter = doc! {
            "name": id
        };

        let user_doc = self.col.find_one(filter, None).ok().expect("error getting user");

        Ok(user_doc.unwrap())
    }
}