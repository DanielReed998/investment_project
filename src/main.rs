mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let user_routes = routes::user_routes(db);

    warp::serve(user_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}