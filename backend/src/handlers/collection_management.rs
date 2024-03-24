use std::convert::Infallible;
use warp::reply::Reply;

use crate::domain::{self, domain_models::InitCollectionDatabase};

use super::handlers_models;

pub async fn init_collections(options : handlers_models::InitCollection) -> Result<impl Reply, Infallible> {
    println!("Creating collection ! {:?}", &options.name);
    let database_collection = InitCollectionDatabase {
        name: options.name
    };
    return domain::database::init_collection(database_collection).await;
}