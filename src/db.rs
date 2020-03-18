use crate::error::HopperError;
use bson::{doc, Bson};
use mongodb::{options::ClientOptions, Client};
pub fn connect_to_db() -> Result<mongodb::Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
    client_options.app_name = Some("hopper-rust-db-service".to_string());
    // Create the client
    Client::with_options(client_options)
}

fn not_found() -> HopperError {
    HopperError {
        status_code: hyper::StatusCode::NOT_FOUND,
    }
}
pub fn find_original_url(client: Client, alias: &str) -> Result<String, HopperError> {
    let result = client
        .database("hopper_db")
        .collection("urls")
        .find_one(doc! { "alias" : alias }, None);

    match result {
        Ok(document) => match document {
            Some(existing_doc) => {
                return existing_doc
                    .get("original")
                    .and_then(Bson::as_str)
                    .ok_or(not_found())
                    .map(|v| String::from(v));
            }
            _ => Err(not_found()),
        },
        _ => Err(not_found()),
    }
}
