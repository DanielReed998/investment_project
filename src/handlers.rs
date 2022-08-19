use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::models::User;
use crate::db::Db;

pub async fn list_users(db: Db) -> Result<impl warp::Reply, Infallible> {
    let users = db.lock().await;
    let users: Vec<User> = users.clone();
    Ok(warp::reply::json(&users))
}

pub async fn create_user(
    new_user: User,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;

    for user in users.iter() {
        if user.id == new_user.id {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    users.push(new_user);

    Ok(StatusCode::CREATED)
}

pub async fn get_user(id: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let users = db.lock().await;
    for user in users.iter() {
        if user.id == id {
            return Ok(Box::new(warp::reply::json(&user)))
        }
    }
    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_user(id: String, updated_user: User, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;
    for user in users.iter_mut() {
        if user.id == id {
            *user = updated_user;
            return Ok(StatusCode::OK);
        }
    }
    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_user(id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;
    let user_count = users.len();

    users.retain(|user| user.id != id);

    let deleted = users.len() != user_count;

    if deleted {
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

