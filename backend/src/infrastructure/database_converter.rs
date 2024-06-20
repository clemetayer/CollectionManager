use crate::infrastructure::database_models::CollectionDatabase;
use backend::models::*;

pub fn convert_collection_list_model_to_database(
    collection_model_list: Vec<Collection>,
) -> Vec<CollectionDatabase> {
    collection_model_list
        .into_iter()
        .map(|collection| convert_collection_model_to_database(collection))
        .collect::<Vec<_>>()
}

fn convert_collection_model_to_database(collection_model: Collection) -> CollectionDatabase {
    let collection_database = CollectionDatabase {
        name: collection_model.name,
        deezer_id: collection_model.deezer_id,
        url: collection_model.url,
    };
    return collection_database;
}
