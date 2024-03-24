use std::convert::Infallible;
use backend::{create_collection, establish_connection};
use warp::reply::Reply;

use super::domain_models::InitCollectionDatabase;

pub async fn init_collection(options : InitCollectionDatabase) -> Result<impl Reply, Infallible> {
    println!("Creating collection ! {:?}", &options.name);
    let connection = &mut establish_connection();
    create_collection(connection,&options.name);
    return Ok(warp::reply());
}