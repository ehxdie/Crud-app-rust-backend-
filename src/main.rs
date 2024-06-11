use warp::{http::StatusCode, Filter};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use tokio::task;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    match connect_to_mongo(&client_uri).await {
        Ok(client) => {
            // MongoDB connection successful, start the Warp server
            println!("Successfully connected to MongoDB");
            // Spawn the Warp server task
            task::spawn(async move {
                warp_server().await;
            });
            // Await the task and propagate any error
        }
        Err(e) => {
            // MongoDB connection failed, log the error
            println!("Failed to connect to MongoDB: {}", e);
        }
    }
   
   
   Ok(())
   

}

async fn warp_server() {
        //  Setting route
   let health_route = warp::path!("health")
         .map(|| StatusCode::OK);
   let routes = health_route
         .with(warp::cors().allow_any_origin());
     warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
     
}


async fn connect_to_mongo(uri: &str) -> Result<Client, Box<dyn Error>> {

    // Create MongoDB client options with a custom DNS resolver
    // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;
   
   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }
   // Get the 'movies' collection from the 'sample_mflix' database:
   let workouts = client.database("test").collection("workouts");

   let new_doc = doc! {
    "title": "Parasite",
    "rep": 2020,
    "load": 10
    };

    let insert_result = workouts.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    Ok(client)
}