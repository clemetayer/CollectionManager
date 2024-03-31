use super::handlers_models::{self, CollectionListElement};
use crate::domain::{self, domain_models::InitCollectionDatabase};
use crate::handlers::errors::*;

pub fn init_collections(options: handlers_models::InitCollection) -> Result<usize, HandlerError> {
    let database_collection = InitCollectionDatabase { name: options.name };
    println!("Initializing collection");
    match domain::database::init_collection(database_collection) {
        Ok(size) => Ok(size),
        Err(e) => {
            eprintln!("Error while initializing a collection in the database");
            Err(HandlerError::HandlerDatabaseError(e))
        }
    }
}

pub fn list_collections() -> Result<Vec<CollectionListElement>, HandlerError> {
    println!("listing collections");
    match domain::database::list_collections() {
        Ok(collections_database) => {
            let collections_handler = collections_database
                .into_iter()
                .map(|collection| {
                    let collection_element = CollectionListElement {
                        name: collection.name,
                    };
                    return collection_element;
                })
                .collect::<Vec<_>>();
            Ok(collections_handler)
        }
        Err(e) => {
            eprintln!("Error while fetching the collections from the database");
            Err(HandlerError::HandlerDatabaseError(e))
        }
    }
}
