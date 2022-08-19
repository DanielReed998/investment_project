use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::User;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// GET /users
fn users_list(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_users)
}

// POST /users
fn create_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_user)
}

// GET /users/{id}
fn get_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_user)
}

// PUT /users/{id}
fn update_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_user)
}

// DELETE /users/{id}
fn delete_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_user)
}

pub fn user_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_user(db.clone())
    .or(create_user(db.clone()))
    .or(update_user(db.clone()))
    .or(delete_user(db.clone()))
    .or(users_list(db))
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024*16)
        .and(warp::body::json())
}

