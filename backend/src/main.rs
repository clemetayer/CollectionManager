mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let routes = routes::build_routes();
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}