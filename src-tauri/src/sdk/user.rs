#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_results)]

use super::prelude::*;

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

#[derive(Clone, Serialize)]
struct Payload {
    caption: String,
    message: Option<String>,
}

#[tauri::command]
pub async fn qr_login(
    window: Window,
    user_state: State<'_, UserState>,
    did: State<'_, DidState>,
    token_state: State<'_, TokenState>,
) -> Result<(), ()> {
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
    let client = HyperClient::builder().build::<_, hyper::Body>(https);

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

    *token_state.write().await = Some(get_token(&user, &did.to_string()).await);

    *user_state.write().await = Some(user);

    Ok(())
}

async fn get_token(user: &User, did: &String) -> Token {
    let https = HttpsConnector::new();
    let client = HyperClient::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://id.app.acfun.cn/rest/app/token/get")
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .header(hyper::header::COOKIE, user.acfun_cookie(did))
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .body(hyper::Body::from("sid=acfun.midground.api"))
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let json: Token = serde_json::from_reader(body.reader()).unwrap();

    if json.result != 0 {
        panic!("{:?}", json)
    }

    json
}
