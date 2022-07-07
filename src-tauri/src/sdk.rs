mod danmaku;
pub mod db;
mod live;
pub mod user;

use tauri::State;

use std::sync::Mutex;

use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: i64,
    pub username: String,
    pub avatar: String,
    pub passtoken: String,
    pub did: String,
}

impl User {
    pub fn kuaishou_cookie(&self, token: &Token) -> String {
        format!(
            "acfun.midground.api_st={}; userId={}",
            token.st, self.userid
        )
    }

    pub fn acfun_cookie(&self) -> String {
        format!(
            "_did=acfun_live_toolbox_{}; acPasstoken={}; auth_key={}",
            self.did, self.passtoken, self.userid
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub result: i32,
    pub error_msg: String,
    pub userId: i64,
    pub ssecurity: String,

    #[serde(rename(deserialize = "acfun.midground.api_st"))]
    pub st: String,

    #[serde(rename(deserialize = "acfun.midground.api_at"))]
    pub at: String,
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

#[tauri::command]
pub async fn check_live_auth(state: State<'_, Mutex<Option<User>>>) -> bool {
    match &*state.lock().unwrap() {
        Some(user) => {
            let token = user::get_token(user).await;
            let auth = live::get_author_auth(&user, &token).await;
            if auth.result == 1 {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

#[tauri::command]
pub async fn check_live_status(state: State<'_, Mutex<Option<User>>>) -> Option<String> {
    match &*state.lock().unwrap() {
        Some(user) => {
            let token = user::get_token(user).await;
            let status = live::get_stream_status(&user, &token).await;
            if status.result == 1 {
                Some(status.data.liveId)
            } else {
                None
            }
        }
        None => None,
    }
}

#[tauri::command]
pub async fn get_stream_config(state: State<'_, Mutex<Option<User>>>) -> Option<String> {
    match &*state.lock().unwrap() {
        Some(user) => {
            let token = user::get_token(&user).await;
            let config = live::get_stream_config(user, &token).await;

            if config.result == 1 {
                Some(config.data.streamPushAddress[0].clone())
            } else {
                None
            }
        }
        None => None,
    }
}

#[derive(Clone, Serialize)]
struct Payload<T> {
    caption: String,
    message: T,
}

#[tauri::command]
pub async fn start_push(
    app: tauri::AppHandle,
    window: tauri::Window,
    state: State<'_, Mutex<Option<User>>>,
) {
    match &*state.lock().unwrap() {
        Some(user) => {
            let token = user::get_token(&user).await;
            let push = live::start_push(user, &token).await;

            if push.result == 1 {
                danmaku::start(user, &token, push.data, |msg_type, payload| {
                    // app.emit_all(
                    //     "danmkaku",
                    //     Payload {
                    //         caption: "".to_string(),
                    //         message: Some("".to_string()),
                    //     },).unwrap());
                    match msg_type {
                        danmaku::r#enum::push_message::action_signal::COMMENT => {}
                        _ => {}
                    }
                })
                .await;
            }
        }
        None => {}
    }
}
