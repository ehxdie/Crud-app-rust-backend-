use warp::{http::StatusCode, Filter};
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection, error, options::{ClientOptions, ResolverConfig}};
use serde::{Serialize, Deserialize};
use std::env;
use std::error::Error;
use tokio;
use tokio::task;
use chrono::{TimeZone, Utc};
use futures_util::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]


// Setting up schema (kind of)
struct Workout {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    reps: i32,
    load: i32,
}

const COLLECTION: &str = "workouts";

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
                warp_server().await;
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
   

    // let insert_result = workouts.insert_one(new_doc.clone(), None).await?;
    // println!("New document ID: {}", insert_result.inserted_id);

    Ok(client)
}

async fn get_workouts(client: &Client) -> Result<Vec<Workout>, Box<dyn Error>> {
    let collection: Collection<Workout> = client.database("test").collection(COLLECTION);
    let cursor = collection.find(None, None).await?;
    let workouts: Vec<Workout> = cursor.try_collect().await?;
    Ok(workouts)
}


async fn get_one_workout(client: &Client, id: &str) -> Result<Option<Workout>, Box<dyn Error>> {
    let collection: Collection<Workout> = client.database("test").collection(COLLECTION);
    if let Ok(object_id) = ObjectId::parse_str(id) {
        let filter = doc! { "_id": object_id };
        let workout = collection.find_one(filter, None).await?;
        Ok(workout)
    } else {
        Ok(None)
    }
}

async fn create_workout(client: &Client, workout: Workout) -> Result<Workout, Box<dyn Error>> {
    let collection: Collection<Workout> = client.database("test").collection(COLLECTION);
    let insert_result = collection.insert_one(workout, None).await?;
    let inserted_id = insert_result.inserted_id.as_object_id().unwrap();
    let inserted_workout = collection.find_one(doc! { "_id": inserted_id }, None).await?.unwrap();
    Ok(inserted_workout)
}

async fn delete_workout(client: &Client, id: &str) -> Result<Option<Workout>, Box<dyn Error>> {
    let collection: Collection<Workout> = client.database("test").collection(COLLECTION);
    if let Ok(object_id) = ObjectId::parse_str(id) {
        let filter = doc! { "_id": object_id };
        let workout = collection.find_one_and_delete(filter, None).await?;
        Ok(workout)
    } else {
        Ok(None)
    }
}

async fn update_workout(client: &Client, id: &str, update: Workout) -> Result<Option<Workout>, Box<dyn Error>> {
    let collection: Collection<Workout> = client.database("test").collection(COLLECTION);
    if let Ok(object_id) = ObjectId::parse_str(id) {
        let filter = doc! { "_id": object_id };
        let update_doc = doc! {
            "$set": {
                "title": update.title,
                "reps": update.reps,
                "load": update.load,
            }
        };
        let updated_workout = collection.find_one_and_update(filter, update_doc, None).await?;
        Ok(updated_workout)
    } else {
        Ok(None)
    }
}
