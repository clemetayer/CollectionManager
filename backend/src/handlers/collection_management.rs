use crate::domain::{self, domain_models::InitCollectionDatabase};

use super::handlers_models::{self, CollectionListElement};

pub fn init_collections(options : handlers_models::InitCollection) {
    println!("Creating collection ! {:?}", &options.name);
    let database_collection = InitCollectionDatabase {
        name: options.name
    };
    domain::database::init_collection(database_collection);
}

pub fn list_collections() -> Vec<CollectionListElement> {
    return domain::database::list_collections()
        .into_iter()
        .map(|collection|{
            let collection_element = CollectionListElement {
                name: collection.name
            };
            return collection_element;
        })
        .collect::<Vec<_>>();
}