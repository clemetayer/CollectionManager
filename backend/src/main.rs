use std::env;

use log::info;

mod api;
mod common;
mod domain;
mod handlers;

#[tokio::main]
async fn main() {
    env_logger::init();
    for arg in env::args().into_iter() {
        info!("arg : {}", arg);
    }
    let routes = api::routes::build_routes();
    info!("Server started at http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
