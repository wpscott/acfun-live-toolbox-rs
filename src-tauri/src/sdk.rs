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
pub async fn is_login(user_state: State<'_, UserState>) -> Result<bool, ()> {
    Ok(match *user_state.read().await {
        Some(_) => true,
        None => false,
    })
}

#[tauri::command]
pub async fn get_user(user_state: State<'_, UserState>) -> Result<Option<User>, ()> {
    Ok((*user_state.read().await).clone())
}

#[tauri::command]
pub async fn check_live_auth(
    user_state: State<'_, UserState>,
    did: State<'_, DidState>,
    token_state: State<'_, TokenState>,
) -> Result<bool, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let auth = live::get_author_auth(user, token)
                    .await
                    .expect("get_author_auth error");
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
    user_state: State<'_, UserState>,
    did: State<'_, Hyphenated>,
    token_state: State<'_, TokenState>,
) -> Result<Option<String>, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let status = live::get_stream_status(user, token)
                    .await
                    .expect("get_stream_status error");
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
    user_state: State<'_, UserState>,
    did: State<'_, Hyphenated>,
    token_state: State<'_, TokenState>,
) -> Result<Option<String>, ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let config = live::get_stream_config(user, token)
                    .await
                    .expect("get_stream_config error");

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
    caption: u32,
    message: T,
}

#[tauri::command]
pub async fn start_push(
    app: AppHandle,
    window: Window,
    user_state: State<'_, UserState>,
    token_state: State<'_, TokenState>,
    client_state: State<'_, RwLock<Option<Arc<Notify>>>>,
) -> Result<(), ()> {
    match &*client_state.read().await {
        Some(notify) => {
            notify.notify_one();
        }
        None => {}
    }
    *client_state.write().await = None;

    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let start = live::start_push(user, token)
                    .await
                    .expect("start_push error");

                if start.result == 1 {
                    let client = danmaku::Client::new_from_user(user, token, start.data);
                    let notify = client
                        .start(move |msg_type, payload| {
                            match msg_type {
                                danmaku::r#enum::push_message::ACTION_SIGNAL => {
                                    let signal = ZtLiveScActionSignal::parse_from_bytes(&payload).unwrap();
                                    for item in signal.item {
                                        match item.signalType.as_str() {
                                            danmaku::r#enum::push_message::action_signal::COMMENT => {
                                                for payload in item.payload {
                                                    let data  = CommonActionSignalComment::parse_from_bytes(&payload).unwrap();
                                                    app.emit_all("danmaku", Payload { caption: 1001, message: data.content}).unwrap();
                                                }
                                            }
                                            danmaku::r#enum::push_message::action_signal::ENTER_ROOM=>{for payload in item.payload {
                                                let data  = CommonActionSignalUserEnterRoom::parse_from_bytes(&payload).unwrap();
                                                app.emit_all("danmaku", Payload { caption: 1002, message: data.userInfo.unwrap().nickname}).unwrap();
                                            }}
                                            danmaku::r#enum::push_message::action_signal::LIKE=>{for payload in item.payload {
                                                let data  = CommonActionSignalLike::parse_from_bytes(&payload).unwrap();
                                                app.emit_all("danmaku", Payload { caption: 1003, message: data.userInfo.unwrap().nickname}).unwrap();
                                            }}
                                            danmaku::r#enum::push_message::action_signal::FOLLOW=>{for payload in item.payload {
                                                let data  = CommonActionSignalUserFollowAuthor::parse_from_bytes(&payload).unwrap();
                                                app.emit_all("danmaku", Payload { caption: 1004, message: data.userInfo.unwrap().nickname}).unwrap();
                                            }}
                                            danmaku::r#enum::push_message::action_signal::GIFT=>{for payload in item.payload {
                                                let data  = CommonActionSignalGift::parse_from_bytes(&payload).unwrap();
                                                app.emit_all("danmaku", Payload { caption: 1005, message: data.userInfo.unwrap().nickname}).unwrap();
                                            }}
                                            danmaku::r#enum::push_message::action_signal::THROW_BANANA=>{}
                                            _ => {}
                                        }
                                    }
                                }
                                danmaku::r#enum::push_message::STATE_SIGNAL => {

                                }
                                danmaku::r#enum::push_message::NOTIFY_SIGNAL => {

                                }
                                _ => {}
                            }
                        })
                        .await;
                    *client_state.write().await = Some(notify);
                }
            }
            None => {}
        },
        None => {}
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_push(
    app: AppHandle,
    window: Window,
    live_id: String,
    user_state: State<'_, UserState>,
    token_state: State<'_, TokenState>,
    client_state: State<'_, ClientState>,
) -> Result<(), ()> {
    match &*user_state.read().await {
        Some(user) => match &*token_state.read().await {
            Some(token) => {
                let stop = live::stop_push(user, token, &live_id)
                    .await
                    .expect("stop_push error");

                if stop.result == 1 {
                    match &*client_state.read().await {
                        Some(notify) => {
                            notify.notify_one();
                        }
                        None => {}
                    }
                    *client_state.write().await = None;
                }
            }
            None => {}
        },
        None => {}
    }

    Ok(())
}
