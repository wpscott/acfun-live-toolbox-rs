use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub result: i32,
    pub data: T,
    host: String,
    pub error_msg: String,
}

pub type Auth = Response<AuthData>;
pub type StreamConfig = Response<ConfigData>;
pub type StartPush = Response<StartPushData>;
pub type StopPush = Response<StopPushData>;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub authStatus: String,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    bizCustomData: String,
    bizUnit: String,
    caption: String,
    liveId: String,
    panoramic: bool,
    streamName: String,
    cover: Vec<Cover>,
}

#[derive(Debug, Deserialize)]
pub struct Cover {
    cdn: String,
    pub url: String,
    urlPattern: String,
    freeTraffic: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigData {
    intervalMillis: i32,
    panoramic: bool,
    pub streamName: String,
    pub streamPullAddress: String,
    pub streamPushAddress: String,
}

#[derive(Debug, Deserialize)]
pub struct StartPushData {
    videoPushRes: String,
    pub liveId: String,
    pub enterRoomAttach: String,
    pub availableTickets: Vec<String>,
    notices: Vec<PushNotice>,
    ticketRetryCount: i16,
    ticketRetryIntervalMs: i32,

    #[serde(rename(deserialize="_config"))]
    config: PushConfig,
}

#[derive(Debug, Deserialize)]
pub struct PushNotice {
    pub userId: i64,
    pub userName: String,
    userGender: String,
    pub notice: String,
}

#[derive(Debug, Deserialize)]
pub struct PushConfig {
    giftSlotSize: i16,
}

#[derive(Debug, Deserialize)]
pub struct StopPushData {
    pub durationMs: i64,
    pub endReason: String,
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
