use crate::config::connections::MongoBD;
use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{ InsertOneResult,UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::ports::outbound::book::Book;


pub struct MongoRepo {
    col: Collection<Book>,
}


impl MongoBD for MongoRepo {
    fn init() -> Self {
        
    
        let client = Client::with_uri_str("mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority").unwrap();
        let db = client.database("rustDB");
        let col: Collection<Book> = db.collection("Book");
        print!("connection start {:?}", &col);
        MongoRepo { col }
    }

    fn create(&self, new_book: Book) -> Result<InsertOneResult, Error> {
        let new_doc: Book = new_book;
        let book = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        
        Ok(book)
    }

    fn update(&self, id: String, new_doc: &Book) ->  Result<UpdateResult, Error> {
        let obj_id = id;
        let filter = doc! { "name": obj_id};
        let doc = doc! {
            "$set":
            {
                "book_name": &new_doc.book_name,
                "book_description": &new_doc.description,
            },
        };

        let update_doc = self.col.update_one(filter, doc, None).ok().expect("error to update");

        Ok(update_doc)
        
    }

    fn delete(&self, id:  String) ->  Result<DeleteResult, Error> {
        let obj_id = id ;
        let filter= doc! {"name":obj_id};

        let user_detail = self.col.delete_one(filter, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }

    fn get(&self, name:  String) ->  Result<Book, Error> {
        let filter = doc! {
            "book_name": name
        };

        let user_doc = self.col.find_one(filter, None).ok().expect("error getting user");

        Ok(user_doc.unwrap())
    }
}