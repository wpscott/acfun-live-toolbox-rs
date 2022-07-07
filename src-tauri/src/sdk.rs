mod danmaku;
pub mod db;
mod live;
pub mod user;

pub mod prelude;

use prelude::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: i64,
    pub username: String,
    pub avatar: String,
    pub passtoken: String,
}

impl User {
    pub fn kuaishou_cookie(&self, token: &Token) -> String {
        format!(
            "acfun.midground.api_st={}; userId={}",
            token.st, self.userid
        )
    }

    pub fn acfun_cookie(&self, did: &String) -> String {
        format!(
            "_did=acfun_live_toolbox_{}; acPasstoken={}; auth_key={}",
            did, self.passtoken, self.userid
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
pub async fn is_login(user_state: State<'_, RwLock<Option<User>>>) -> Result<bool, ()> {
    Ok(match *user_state.read().await {
        Some(_) => true,
        None => false,
    })
}

#[tauri::command]
pub async fn get_user(user_state: State<'_, RwLock<Option<User>>>) -> Result<Option<User>, ()> {
    Ok((*user_state.read().await).clone())
}

#[tauri::command]
pub async fn check_live_auth(
    user_state: State<'_, RwLock<Option<User>>>,
    did: State<'_, Hyphenated>,
    token_state: State<'_, RwLock<Option<Token>>>,
) -> Result<bool, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let auth = live::get_author_auth(user, token).await.unwrap();
                if auth.result == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        },
        None => Ok(false),
    }
}

#[tauri::command]
pub async fn check_live_status(
    user_state: State<'_, RwLock<Option<User>>>,
    did: State<'_, Hyphenated>,
    token_state: State<'_, RwLock<Option<Token>>>,
) -> Result<Option<String>, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let status = live::get_stream_status(user, token).await;
                if status.result == 1 {
                    Ok(Some(status.data.liveId))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        },
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn get_stream_config(
    user_state: State<'_, RwLock<Option<User>>>,
    did: State<'_, Hyphenated>,
    token_state: State<'_, RwLock<Option<Token>>>,
) -> Result<Option<String>, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let config = live::get_stream_config(user, token).await;

                if config.result == 1 {
                    Ok(Some(config.data.streamPushAddress[0].clone()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        },
        None => Ok(None),
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
    user_state: State<'_, RwLock<Option<User>>>,
    token_state: State<'_, RwLock<Option<Token>>>,
    tcp_state: State<'_, Mutex<Option<Arc<TcpStream>>>>,
) -> Result<(), ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let push = live::start_push(user, token).await;

                if push.result == 1 {
                    let tcp = danmaku::start(user, token, push.data, |msg_type, payload| {
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

                    *tcp_state.lock().unwrap() = Some(tcp);
                }
            }
            None => {}
        },
        None => {}
    }

    Ok(())
}
