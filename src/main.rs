use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
pub mod adpters;
pub mod ports;
pub mod config;
pub mod domain;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use crate::adpters::http_in::users;
use std::error::Error;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    let client_uri = "mongodb+srv://admin:Test10@cluster0.ud6jftd.mongodb.net/?retryWrites=true&w=majority";

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
        println!("Databases:");
        for name in client.list_database_names(None, None).await? {
            println!("- {}", name);
        }
    let new_doc = doc! {
        "username":"test",
        "age": 25,
        "description": "a human readable description" ,
        
    };

    println!("{}",new_doc);

    let movies = &client.database("sample_mflix").collection("movies");
    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    let movie = movies.find_one(doc!{
        "username":"test"
    }, None).await?.expect("missing username");

    let update_document = movies.update_one(doc! {
        "_id": &movie.get("_id") 
    },
        doc! {
            "$set": { "username": "luana"}
        }, None,
    ).await?;

    let find_one_delete = movies.find_one_and_delete(doc!{
        "username": "luana"
    },  None).await?;
    
    println!("Updated {} document", update_document.modified_count);
    println!("Delete {:?}",find_one_delete);

    let find_all = movies.find(None, None).await.unwrap();

    println!("{:?}",find_all);

   // build our application with a single route
    let app = Router::new()
        .route("/", get(users::health))
        .route("/users", post(users::create_user))
        .route("/users", put(users::create_user))
        ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

        Ok(())
      
}

