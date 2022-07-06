use tauri::{State, Window};

use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use hyper::{
    body::{aggregate, Buf},
    Client,
};
use hyper_tls::HttpsConnector;

use serde::{Deserialize};
use serde_json;

use super::{db, Payload, User};

#[derive(Debug, Deserialize)]
struct StartResult {
    expireTime: i64,
    imageData: String,
    next: String,
    qrLoginSignature: String,
    qrLoginToken: String,
    result: i32,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ScanResult {
    next: String,
    qrLoginSignature: String,
    status: String,
    result: i32,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AcceptResult {
    next: String,
    qrLoginSignature: String,
    status: String,
    result: i32,
    userId: i64,
    ac_username: String,
    ac_userimg: String,
    acPasstoken: String,
    error_msg: Option<String>,
}

#[tauri::command]
pub async fn qr_login(window: Window, state: State<'_, Mutex<Option<User>>>) -> Result<(), ()> {
    window
        .emit(
            "qr-login",
            Payload {
                caption: String::from("获取二维码中"),
                message: None,
            },
        )
        .unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let res = client
        .get(
            format!(
                "https://scan.acfun.cn/rest/pc-direct/qr/start?type=WEB_LOGIN&_={0}",
                now
            )
            .parse()
            .unwrap(),
        )
        .await
        .unwrap();

    let body = aggregate(res).await.unwrap();

    let start: StartResult = serde_json::from_reader(body.reader()).unwrap();
    window
        .emit(
            "qr-login",
            Payload {
                caption: String::from("请使用AcFun App扫码登录"),
                message: Some(format!("data:image/png;base64,{}", start.imageData)),
            },
        )
        .unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let res = client.get(format!("https://scan.acfun.cn/rest/pc-direct/qr/{0}?qrLoginToken={1}&qrLoginSignature={2}&_={3}", start.next,
    start.qrLoginToken,
    start.qrLoginSignature, now).parse().unwrap()).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let accept: ScanResult = serde_json::from_reader(body.reader()).unwrap();
    window
        .emit(
            "qr-login",
            Payload {
                caption: String::from("已扫码，请在AcFun App上确认登录"),
                message: None,
            },
        )
        .unwrap();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let res = client.get(format!("https://scan.acfun.cn/rest/pc-direct/qr/{0}?qrLoginToken={1}&qrLoginSignature={2}&_={3}", accept.next,
            start.qrLoginToken,
            accept.qrLoginSignature, now).parse().unwrap()).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let confirm: AcceptResult = serde_json::from_reader(body.reader()).unwrap();
    window
        .emit(
            "qr-login",
            Payload {
                caption: String::from(format!("成功登录，欢迎回来 {}", confirm.ac_username)),
                message: None,
            },
        )
        .unwrap();

    let user = User {
        userid: confirm.userId,
        username: confirm.ac_username,
        avatar: confirm.ac_userimg,
        passtoken: confirm.acPasstoken,
    };

    db::save_user(&user).unwrap();

    *state.lock().unwrap() = Some(user);

    Ok(())
}
