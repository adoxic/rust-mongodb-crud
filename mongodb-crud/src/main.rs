use mongodb::bson::{self, doc, Bson};
use std::env;
use std::error::Error;
use tokio;

//Practice typing out the Rust Crud Tutorial by Mark Smith https://developer.mongodb.com/quickstart/rust-crud-tutorial/

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

    // Get the movie
    let movie = movies
        .find_one(
            doc! {
                    "title": "Parasite"
            },
            None,
        ).await?
        .expect("Missing 'Parasite' document.");
    println!("Movie: {}", movie);

    // Update the document:
    let update_result = movies.update_one(
        doc! {
            "_id": &insert_result.inserted_id,
        },
        doc! {
            "$set": { "year": 2019 }
        },
        None,
    ).await?;
    println!("Updated {} document", update_result.modified_count);

    // Look up the document again to confirm it's been updated:
    let movie = movies
        .find_one(
        doc! {
                "_id": &insert_result.inserted_id,
        },
        None,
        ).await?
        .expect("Missing 'Parasite' document.");
    println!("Updated Movie: {}", &movie);

    // Delete all documents for movies called "Parasite":
    let delete_result = movies.delete_many(
        doc! {
        "title": "Parasite"
        },
        None,
    ).await?;
    println!("Deleted {} documents", delete_result.deleted_count);

    Ok(())
}
