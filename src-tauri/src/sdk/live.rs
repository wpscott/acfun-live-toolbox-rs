use hyper::{
    body::{aggregate, Buf},
    Body, Client, Method, Request,
};
use hyper_tls::HttpsConnector;
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;
use serde_json;

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

use rand::prelude::*;

use super::{Token, User, VERSION};

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub result: i32,
    pub data: T,
    host: String,
    pub error_msg: String,
}

pub type Auth = Response<AuthData>;
pub type StreamConfig = Response<ConfigData>;
pub type StreamStatus = Response<StatusData>;
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
    pub liveId: String,
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
    pub streamPushAddress: Vec<String>,
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

    #[serde(rename(deserialize = "_config"))]
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

const KUAISHOU_ZT: &str = "https://api.kuaishouzt.com";
const AUTHOR_AUTH: &str = "/rest/zt/live/authorAuth";
const LIVE_CONFIG: &str = "/rest/zt/live/web/obs/config";
const LIVE_STATUS: &str = "/rest/zt/live/web/obs/status";
const START_PUSH: &str = "/rest/zt/live/startPush";
const STOP_PUSH: &str = "/rest/zt/live/stopPush";
const GIFT_ALL: &str = "/rest/zt/live/gift/all";

const ACFUN_MEMBER: &str = "https://member.acfun.cn/";

union Nonce {
    random: u32,
    now: u64,
    result: i64,
}

#[inline]
fn generate_nonce() -> i64 {
    let mut nonce = Nonce {
        now: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 60,
    };

    nonce.random = thread_rng().gen::<u32>();

    unsafe {
        return nonce.result;
    }
}

fn sign(url: &str, token: &String, extra: Option<BTreeMap<&str, &str>>) -> (String, String) {
    let mut hmac = HmacSha256::new_from_slice(&base64::decode(token).unwrap()).unwrap();

    let mut _query = BTreeMap::from([
        ("appver", "1.9.0.200"),
        ("sys", "PC_10"),
        ("kpn", "ACFUN_APP.LIVE_MATE"),
        ("kpf", "WINDOWS_PC"),
        ("subBiz", "mainApp"),
    ]);
    let query = (match extra {
        Some(data) => {
            _query.extend(data);
            _query
        }
        None => _query,
    })
    .iter()
    .map(|(key, value)| format!("{}={}", key, value))
    .collect::<Vec<String>>()
    .join("&");

    let nonce = generate_nonce();

    hmac.update(format!("POST&{}&{}&{}", url, query, nonce).as_bytes());

    let hash = hmac.finalize();

    let mut sign = vec![0u8; 40];

    sign.extend(&nonce.to_be_bytes());
    sign.extend(&hash.into_bytes());

    (query, base64::encode_config(sign, base64::URL_SAFE_NO_PAD))
}


pub async fn get_author_auth(user: &User, token: &Token) -> Auth {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (query, sign) = sign(AUTHOR_AUTH, &token.ssecurity, None);

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "{}{}?{}&__clientSign={}",
            KUAISHOU_ZT, AUTHOR_AUTH, query, sign
        ))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(Body::default())
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: Auth = serde_json::from_reader(body.reader()).unwrap();

    result
}


pub async fn get_stream_config(user: &User, token: &Token) -> StreamConfig {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let form = format!(
        "kpf=WINDOWS_PC&kpn=ACFUN_APP.LIVE_MATE&subBiz=mainApp&userId={}&acfun.midground.api_st={}",
        token.userId, token.st
    );
    // let form = urlencoding::encode(form.as_str()).into_owned();

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("{}{}", KUAISHOU_ZT, LIVE_CONFIG))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .header(hyper::header::REFERER, ACFUN_MEMBER)
        .body(Body::from(form))
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: StreamConfig = serde_json::from_reader(body.reader()).unwrap();

    result
}


pub async fn get_stream_status(user: &User, token: &Token) -> StreamStatus {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let form = format!(
        "kpf=WINDOWS_PC&kpn=ACFUN_APP.LIVE_MATE&subBiz=mainApp&userId={}&acfun.midground.api_st={}",
        token.userId, token.st
    );
    // let form = urlencoding::encode(form.as_str()).into_owned();

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("{}{}", KUAISHOU_ZT, LIVE_STATUS))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .header(hyper::header::REFERER, ACFUN_MEMBER)
        .body(Body::from(form))
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: StreamStatus = serde_json::from_reader(body.reader()).unwrap();

    result
}


pub async fn start_push(user: &User, token: &Token) -> StartPush {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (query, sign) = sign(START_PUSH, &token.ssecurity, None);

    // TODO

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "{}{}?{}&__clientSign={}",
            KUAISHOU_ZT, START_PUSH, query, sign
        ))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "multipart/form-data",
        )
        .body(Body::default())
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: StartPush = serde_json::from_reader(body.reader()).unwrap();

    result
}


pub async fn stop_push(user: &User, token: &Token, live_id: &str) -> StopPush {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (query, sign) = sign(
        STOP_PUSH,
        &token.ssecurity,
        Some(BTreeMap::from([("liveId", live_id)])),
    );

    // TODO

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "{}{}?{}&__clientSign={}",
            KUAISHOU_ZT, STOP_PUSH, query, sign
        ))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(hyper::header::CONTENT_TYPE, "multipart/form-data")
        .body(Body::default())
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: StopPush = serde_json::from_reader(body.reader()).unwrap();

    result
}

type Gift = Response<GiftData>;

#[derive(Debug, Deserialize)]
pub struct GiftData {
    #[serde(rename = "giftList")]
    gift_list: Vec<GiftList>,
    #[serde(rename = "externalDisplayGiftId")]
    external_display_gift_id: i64,
    #[serde(rename = "externalDisplayGiftTipsDelayTime")]
    external_display_gift_tips_delay_time: i64,
    #[serde(rename = "externalDisplayGift")]
    external_display_gift: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GiftList {
    #[serde(rename = "giftId")]
    gift_id: i64,
    #[serde(rename = "giftName")]
    gift_name: String,
    #[serde(rename = "arLiveName")]
    ar_live_name: ArLiveName,
    #[serde(rename = "payWalletType")]
    pay_wallet_type: i64,
    #[serde(rename = "giftPrice")]
    gift_price: i64,
    #[serde(rename = "webpPicList")]
    webp_pic_list: Vec<PicList>,
    #[serde(rename = "pngPicList")]
    png_pic_list: Vec<PicList>,
    #[serde(rename = "smallPngPicList")]
    small_png_pic_list: Vec<PicList>,
    #[serde(rename = "allowBatchSendSizeList")]
    allow_batch_send_size_list: Vec<i64>,
    #[serde(rename = "canCombo")]
    can_combo: bool,
    #[serde(rename = "canDraw")]
    can_draw: bool,
    #[serde(rename = "magicFaceId")]
    magic_face_id: i64,
    #[serde(rename = "vupArId")]
    vup_ar_id: i64,
    description: Option<String>,
    #[serde(rename = "redpackPrice")]
    redpack_price: i64,
    #[serde(rename = "cornerMarkerText")]
    corner_marker_text: CornerMarkerText,
}

#[derive(Debug, Deserialize)]
pub struct PicList {
    cdn: Cdn,
    url: String,
    #[serde(rename = "urlPattern")]
    url_pattern: String,
    #[serde(rename = "freeTraffic")]
    free_traffic: bool,
}

#[derive(Debug, Deserialize)]
pub enum ArLiveName {
    #[serde(rename = "ac102")]
    Ac102,
    #[serde(rename = "ac104")]
    Ac104,
    #[serde(rename = "ac106")]
    Ac106,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Deserialize)]
pub enum CornerMarkerText {
    #[serde(rename = "节日")]
    CornerMarkerText,
    #[serde(rename = "活动")]
    Empty,
    #[serde(rename = "专属")]
    Fluffy,
    #[serde(rename = "春晚")]
    Purple,
    #[serde(rename = "涂鸦")]
    Sticky,
    #[serde(rename = "")]
    Tentacled,
}

#[derive(Debug, Deserialize)]
pub enum Cdn {
    #[serde(rename = "blobStore")]
    BlobStore,
}

pub async fn get_gift_list(user: &User, token: &Token) -> Gift {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (query, sign) = sign(STOP_PUSH, &token.ssecurity, None);

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "{}{}?{}&__clientSign={}",
            KUAISHOU_ZT, GIFT_ALL, query, sign
        ))
        .header(hyper::header::COOKIE, user.kuaishou_cookie(token))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(Body::default())
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let result: Gift = serde_json::from_reader(body.reader()).unwrap();

    result
}
