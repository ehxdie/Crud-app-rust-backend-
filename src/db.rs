use crate::response::{NoteData, NoteListResponse, NoteResponse, SingleNoteResponse};
use crate::{
    error::Error::*, model::NoteModel, schema::CreateNoteSchema, schema::UpdateNoteSchema, Result,
};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

const COLLECTION: &str = "workouts";

pub async fn connect_to_mongo(uri: &str) -> Result<Client, Box<dyn Error>> {

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