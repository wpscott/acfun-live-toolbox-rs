pub mod danmaku;
pub mod db;
pub mod live;
pub mod user;

use tauri::{State};

use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: i64,
    pub username: String,
    pub avatar: String,
    pub passtoken: String,
}

#[derive(Clone, Serialize)]
struct Payload {
    caption: String,
    message: Option<String>,
}

#[tauri::command]
pub fn is_login(state: State<Mutex<Option<User>>>) -> bool {
    match *state.lock().unwrap() {
        Some(_) => true,
        None => false,
    }
}

#[tauri::command]
pub fn get_user(state: State<Mutex<Option<User>>>) -> Option<User> {
    (*state.lock().unwrap()).clone()
}
