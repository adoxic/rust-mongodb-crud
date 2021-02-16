use mongodb::bson::{self, doc, Bson};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an env variable:
    let client_uri = env::var("MONGODB_URI").expect("remember to set this");

    // A Client is needed to connect to MongoDB:
    let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;

    // Print the database in our MongoDB cluster:
    println!("Database:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
    };

    // Get the 'Movies' collection:
    let movies = client.database("sample_mflix").collection("movies");

    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    Ok(())
}
