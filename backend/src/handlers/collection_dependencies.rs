use crate::{
    domain::{
        database::{add_collection_to_parent, get_collection_id_by_deezer_id},
        errors::DatabaseDomainError,
    },
    handlers::errors::HandlerError,
};

use super::collection_commons::{convert_string_to_u64, create_collection_from_playlist};

pub async fn add_collection_dependency(
    parent_deezer_id: String,
    child_deezer_id: String,
) -> Result<bool, HandlerError> {
    add_collection_if_not_in_database(parent_deezer_id.clone()).await?;
    add_collection_if_not_in_database(child_deezer_id.clone()).await?;
    add_collection_dependency_to_database(parent_deezer_id, child_deezer_id)?;
    return Ok(true);
}

fn add_collection_dependency_to_database(
    parent_deezer_id: String,
    child_deezer_id: String,
) -> Result<bool, HandlerError> {
    let parent_id: i32;
    let child_id: i32;
    match get_collection_id_by_deezer_id(parent_deezer_id.clone()) {
        Ok(id) => {
            parent_id = id;
        }
        Err(e) => {
            eprintln!(
                "Error while getting collection id {} : {:?}",
                parent_deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    match get_collection_id_by_deezer_id(child_deezer_id.clone()) {
        Ok(id) => {
            child_id = id;
        }
        Err(e) => {
            eprintln!(
                "Error while getting collection id {} : {:?}",
                child_deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    match add_collection_to_parent(parent_id, child_id) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Error while adding collection {} to collection {}",
                parent_id, child_id
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
    return Ok(true);
}

async fn add_collection_if_not_in_database(deezer_id: String) -> Result<bool, HandlerError> {
    match get_collection_id_by_deezer_id(deezer_id.clone()) {
        Ok(_) => {
            return Ok(true);
        } // Collection already exists, non need to add it
        Err(e) => match e {
            DatabaseDomainError::ResultError(_) => {
                match create_collection_from_playlist(convert_string_to_u64(
                    &deezer_id.clone().as_str(),
                ))
                .await
                {
                    Ok(_) => return Ok(true),
                    Err(e) => {
                        eprintln!(
                            "Error creating collection from playlist id {} : {:?}",
                            deezer_id, e
                        );
                        return Err(HandlerError::HandlerDeezerError());
                    }
                }
            }
            DatabaseDomainError::ConnectionError() => {
                eprintln!(
                    "Connection error while trying to add the collection {} if not in database",
                    deezer_id
                );
                return Err(HandlerError::HandlerDatabaseError(e));
            }
        },
    }
}
