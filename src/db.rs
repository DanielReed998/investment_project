use std::sync::Arc;
use std::fs::File;

use serde_json::from_reader;
use tokio::sync::Mutex;

use crate::models::User;

pub type Db = Arc<Mutex<Vec<User>>>;

pub fn init_db() -> Db {
    let file = File::open("./data/users.json");
    match file {
        Ok(json) => {
            let users = from_reader(json).unwrap();
            Arc::new(Mutex::new(users))
        },
        Err(_) => {
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}
