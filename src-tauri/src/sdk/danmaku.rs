pub mod r#enum;
mod utils;

use protobuf::{EnumOrUnknown, Message, MessageField};
use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use crate::{
    AppInfo::AppInfo,
    DeviceInfo::{device_info::PlatformType, DeviceInfo},
    HandshakeRequest::HandshakeRequest,
    KeepAliveRequest::KeepAliveRequest,
    PacketHeader::{packet_header::EncryptionMode, PacketHeader},
    RegisterRequest::{
        register_request::{ActiveStatus, PresenceStatus},
        RegisterRequest,
    },
    RegisterResponse::RegisterResponse,
    TokenInfo::{token_info::TokenType, TokenInfo},
    UpstreamPayload::UpstreamPayload,
    ZtCommonInfo::ZtCommonInfo,
    ZtLiveCsCmd::ZtLiveCsCmd,
    ZtLiveCsCmd::ZtLiveCsCmdAck,
    ZtLiveCsEnterRoom::{ZtLiveCsEnterRoom, ZtLiveCsEnterRoomAck},
    ZtLiveCsHeartbeat::ZtLiveCsHeartbeat,
    ZtLiveScMessage::{zt_live_sc_message::CompressionType, ZtLiveScMessage},
    ZtLiveScStatusChanged::{zt_live_sc_status_changed::Type, ZtLiveScStatusChanged},
};

use super::{live::StartPushData, User};

#[derive(Debug, Default)]
pub struct ClientRequest {
    userid: i64,
    service_token: String,
    security_key: [u8; 32],
    live_id: String,
    enter_room_attach: String,
    session_key: [u8; 32],
    tickets: Vec<String>,
    ticket_index: usize,
    instance_id: i64,
    lz4_compression_threshold: i32,
    seq_id: AtomicI64,
    heartbeat_seq_id: AtomicI64,
}

impl ClientRequest {
    const SUB_BIZ: &'static str = "mainApp";
    const RETRY_COUNT: u32 = 1;
    const APP_ID: i32 = 13;
    // const APP_NAME: &'static str = "link-sdk";
    // const SDK_VERSION: &'static str = "1.9.0.200";
    const KPN: &'static str = "ACFUN_APP.LIVE_MATE";
    const KPF: &'static str = "WINDOWS_PC";
    const CLIENT_LIVE_SDK_VERSION: &'static str = "kwai-acfun-live-link";
    const LINK_VERSION: &'static str = "2.13.8";

    pub fn new(
        userid: i64,
        service_token: String,
        security_key: String,
        live_id: String,
        enter_room_attach: String,
        tickets: Vec<String>,
    ) -> ClientRequest {
        ClientRequest {
            userid,
            service_token,
            security_key: utils::convert_key(security_key),
            live_id,
            enter_room_attach,
            tickets,
            ticket_index: 0,
            ..Default::default()
        }
    }

    pub fn register(
        &mut self,
        instance_id: i64,
        session_key: Vec<u8>,
        lz4_compression_htreshold: i32,
    ) {
        self.instance_id = instance_id;
        self.session_key = session_key.try_into().unwrap();
        self.lz4_compression_threshold = lz4_compression_htreshold;
    }

    pub fn get_seq_id(&self) -> i64 {
        self.seq_id.load(Ordering::Relaxed)
    }

    pub fn get_security_key(&self) -> &[u8; 32] {
        &self.security_key
    }

    pub fn get_session_key(&self) -> &[u8; 32] {
        &self.session_key
    }

    pub fn next_ticket(&mut self) -> String {
        self.ticket_index += 1;
        self.tickets[self.ticket_index].clone()
    }

    pub fn handshake_request(&self) -> Vec<u8> {
        let request = HandshakeRequest {
            unknown1: 1,
            unknown2: 1,
            ..Default::default()
        };

        let payload = self.generate_payload(
            r#enum::command::HANDSHAKE,
            Some(request.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        utils::encode(header.write_to_bytes().unwrap(), body, &self.security_key)
    }

    pub fn register_request(&self) -> Vec<u8> {
        let request = RegisterRequest {
            appInfo: MessageField::some(AppInfo {
                sdkVersion: String::from(ClientRequest::CLIENT_LIVE_SDK_VERSION),
                linkVersion: String::from(ClientRequest::LINK_VERSION),
                ..Default::default()
            }),
            deviceInfo: MessageField::some(DeviceInfo {
                platformType: EnumOrUnknown::new(PlatformType::H5_WINDOWS),
                deviceModel: String::from("h5"),
                ..Default::default()
            }),
            ztCommonInfo: MessageField::some(ZtCommonInfo {
                kpn: String::from(ClientRequest::KPN),
                kpf: String::from(ClientRequest::KPF),
                uid: self.userid,
                ..Default::default()
            }),
            presenceStatus: EnumOrUnknown::new(PresenceStatus::kPresenceOnline),
            appActiveStatus: EnumOrUnknown::new(ActiveStatus::kAppInForeground),
            instanceId: self.instance_id,
            ..Default::default()
        };

        let payload = self.generate_payload(
            r#enum::command::REGISTER,
            Some(request.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let mut header = self.generate_header(&body);

        header.tokenInfo = MessageField::some(TokenInfo {
            tokenType: EnumOrUnknown::new(TokenType::kServiceToken),
            token: self.service_token.as_bytes().to_vec(),
            ..Default::default()
        });

        header.encryptionMode = EnumOrUnknown::new(EncryptionMode::kEncryptionServiceToken);

        self.seq_id.fetch_add(1, Ordering::SeqCst);

        utils::encode(header.write_to_bytes().unwrap(), body, &self.security_key)
    }

    pub fn keep_alive_request(&self) -> Vec<u8> {
        let request = KeepAliveRequest {
            presenceStatus: EnumOrUnknown::new(PresenceStatus::kPresenceOnline),
            appActiveStatus: EnumOrUnknown::new(ActiveStatus::kAppInForeground),
            ..Default::default()
        };

        let payload = self.generate_payload(
            r#enum::command::KEEP_ALIVE,
            Some(request.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        self.seq_id.fetch_add(1, Ordering::SeqCst);

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    pub fn enter_room_request(&self) -> Vec<u8> {
        let request = ZtLiveCsEnterRoom {
            enterRoomAttach: self.enter_room_attach.clone(),
            clientLiveSdkVersion: String::from(ClientRequest::CLIENT_LIVE_SDK_VERSION),
            ..Default::default()
        };

        let cmd = self.generate_command(
            r#enum::global_command::ENTER_ROOM,
            Some(request.write_to_bytes().unwrap()),
        );

        let payload = self.generate_payload(
            r#enum::command::GLOBAL_COMMAND,
            Some(cmd.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        self.seq_id.fetch_add(1, Ordering::SeqCst);

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    pub fn push_message_response(&self, header_seq_id: i64) -> Vec<u8> {
        let payload = self.generate_payload(r#enum::command::PUSH_MESSAGE, None);

        let body = payload.write_to_bytes().unwrap();

        let mut header = self.generate_header(&body);

        header.seqId = header_seq_id;

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    pub fn heartbeat_request(&self) -> Vec<u8> {
        let request = ZtLiveCsHeartbeat {
            clientTimestampMs: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            sequence: self.heartbeat_seq_id.load(Ordering::Relaxed),
            ..Default::default()
        };

        let cmd = self.generate_command(
            r#enum::global_command::HEARTBEAT,
            Some(request.write_to_bytes().unwrap()),
        );

        let payload = self.generate_payload(
            r#enum::command::GLOBAL_COMMAND,
            Some(cmd.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        self.heartbeat_seq_id.fetch_add(1, Ordering::SeqCst);
        self.seq_id.fetch_add(1, Ordering::SeqCst);

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    pub fn user_exit_request(&self) -> Vec<u8> {
        let cmd = self.generate_command(r#enum::global_command::USER_EXIT, None);

        let payload = self.generate_payload(
            r#enum::command::GLOBAL_COMMAND,
            Some(cmd.write_to_bytes().unwrap()),
        );

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    pub fn unregister_request(&self) -> Vec<u8> {
        let payload = self.generate_payload(r#enum::command::UNREGISTER, None);

        let body = payload.write_to_bytes().unwrap();

        let header = self.generate_header(&body);

        utils::encode(header.write_to_bytes().unwrap(), body, &&self.session_key)
    }

    fn generate_command(&self, command: &str, msg: Option<Vec<u8>>) -> ZtLiveCsCmd {
        match msg {
            Some(data) => ZtLiveCsCmd {
                cmdType: String::from(command),
                payload: data,
                ticket: self.tickets[self.ticket_index].clone(),
                liveId: self.live_id.clone(),
                ..Default::default()
            },

            None => ZtLiveCsCmd {
                cmdType: String::from(command),
                ticket: self.tickets[self.ticket_index].clone(),
                liveId: self.live_id.clone(),
                ..Default::default()
            },
        }
    }

    fn generate_payload(&self, command: &str, msg: Option<Vec<u8>>) -> UpstreamPayload {
        match msg {
            Some(data) => UpstreamPayload {
                command: String::from(command),
                seqId: self.seq_id.load(Ordering::Relaxed),
                retryCount: ClientRequest::RETRY_COUNT,
                subBiz: String::from(ClientRequest::SUB_BIZ),
                payloadData: data,
                ..Default::default()
            },
            None => UpstreamPayload {
                command: String::from(command),
                seqId: self.seq_id.load(Ordering::Relaxed),
                retryCount: ClientRequest::RETRY_COUNT,
                subBiz: String::from(ClientRequest::SUB_BIZ),
                ..Default::default()
            },
        }
    }

    fn generate_header(&self, msg: &Vec<u8>) -> PacketHeader {
        PacketHeader {
            appId: ClientRequest::APP_ID,
            uid: self.userid,
            instanceId: self.instance_id,
            decodedPayloadLen: msg.len() as u32,
            encryptionMode: EnumOrUnknown::new(EncryptionMode::kEncryptionSessionKey),
            seqId: self.seq_id.load(Ordering::Relaxed),
            kpn: String::from(ClientRequest::KPN),
            ..Default::default()
        }
    }
}

use hyper::{
    body::{aggregate, Buf},
    Client, Method, Request,
};
use hyper_tls::HttpsConnector;

use serde_json;

use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use flate2::read::GzDecoder;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SLINK_HOST: &str = "slink.gifshow.com:14000";

pub async fn start(
    user: &User,
    push_data: StartPushData,
    handler: fn(msgType: &str, payload: Vec<u8>),
) {
    let did = Uuid::new_v4().hyphenated().to_string();
    let acfun_cookie = format!(
        "_did=acfun_live_toolbox_{}; acPasstoken={}; auth_key={};",
        did, user.passtoken, user.userid
    );

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://id.app.acfun.cn/rest/app/token/get")
        .header(
            hyper::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .header(hyper::header::COOKIE, acfun_cookie)
        .header(
            hyper::header::USER_AGENT,
            format!("acfun_live_toolbox {}", VERSION),
        )
        .body(hyper::Body::from("sid=acfun.midground.api"))
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = aggregate(res).await.unwrap();

    let json: serde_json::Value = serde_json::from_reader(body.reader()).unwrap();

    if json.get("result").unwrap().as_i64().unwrap() != 0 {
        panic!("{}", json)
    }

    let service_token = json
        .get("acfun.midground.api_st")
        .unwrap()
        .as_str()
        .unwrap();
    let security_key = json.get("ssecurity").unwrap().as_str().unwrap();

    let mut request = Arc::<ClientRequest>::new(ClientRequest::new(
        user.userid,
        String::from(service_token),
        String::from(security_key),
        push_data.liveId,
        push_data.enterRoomAttach,
        push_data.availableTickets,
    ));

    match TcpStream::connect(SLINK_HOST) {
        Ok(write) => {
            let read = write.try_clone().unwrap();

            // let mut buffer = [0u8; 8192];
            let mut buffer = vec![0u8; 8192];
            let writer = Arc::<Mutex<BufWriter<TcpStream>>>::new(Mutex::new(BufWriter::new(write)));
            let mut reader = BufReader::new(read);

            writer
                .lock()
                .unwrap()
                .write(&request.handshake_request())
                .unwrap();
            writer
                .lock()
                .unwrap()
                .write(&request.register_request())
                .unwrap();

            let timer = timer::Timer::new();
            let mut guard = Option::<timer::Guard>::None;

            loop {
                reader.read_to_end(&mut buffer).unwrap();

                let (header, down) = utils::decode(
                    &buffer,
                    request.get_security_key(),
                    request.get_session_key(),
                );

                match down.command.as_str() {
                    r#enum::command::GLOBAL_COMMAND => {
                        let cmd = ZtLiveCsCmdAck::parse_from_bytes(&down.payloadData).unwrap();
                        match cmd.cmdAckType.as_str() {
                            r#enum::global_command::ENTER_ROOM_ACK => {
                                let ack =
                                    ZtLiveCsEnterRoomAck::parse_from_bytes(&cmd.payload).unwrap();

                                let hbh = Arc::clone(&writer);
                                let copy = Arc::clone(&request);
                                guard = Some(timer.schedule_repeating(
                                    chrono::Duration::milliseconds(ack.heartbeatIntervalMs),
                                    move || {
                                        hbh.lock()
                                            .unwrap()
                                            .write(&copy.heartbeat_request())
                                            .unwrap();
                                    },
                                ));
                            }
                            r#enum::global_command::HEARTBEAT_ACK => {}
                            r#enum::global_command::USER_EXIT_ACK => {}
                            _ => {
                                log::log!(
                                    log::Level::Warn,
                                    "Unhandled Global.ZtLiveInteractive.CsCmdAck: {:?}",
                                    cmd
                                )
                            }
                        }
                    }
                    r#enum::command::PUSH_MESSAGE => {
                        writer
                            .lock()
                            .unwrap()
                            .write(&request.push_message_response(header.seqId))
                            .unwrap();

                        let message = ZtLiveScMessage::parse_from_bytes(&down.payloadData).unwrap();
                        let mut payload = message.payload;
                        if message.compressionType.unwrap() == CompressionType::GZIP {
                            let mut d = GzDecoder::<&[u8]>::new(&payload);
                            let mut de = Vec::new();
                            d.read_to_end(&mut de).unwrap();
                            payload = de;
                        }
                        let msg_type = message.messageType.as_str();
                        match msg_type {
                            r#enum::push_message::ACTION_SIGNAL => {
                                handler(msg_type, payload);
                            }
                            r#enum::push_message::STATE_SIGNAL => {
                                handler(msg_type, payload);
                            }
                            r#enum::push_message::NOTIFY_SIGNAL => {
                                handler(msg_type, payload);
                            }
                            r#enum::push_message::STATUS_CHANGED => {
                                let resp =
                                    ZtLiveScStatusChanged::parse_from_bytes(&payload).unwrap();
                                match resp.type_.unwrap() {
                                    Type::LIVE_CLOSED => {
                                        break;
                                    }
                                    Type::LIVE_BANNED => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            r#enum::push_message::TICKET_INVALID => {
                                Arc::get_mut(&mut request).unwrap().next_ticket();
                                writer
                                    .lock()
                                    .unwrap()
                                    .write(&request.enter_room_request())
                                    .unwrap();
                            }
                            _ => {
                                log::log!(
                                    log::Level::Warn,
                                    "Unhandled Push.ZtLiveInteractive.Message: {:?}",
                                    msg_type
                                )
                            }
                        }
                    }
                    r#enum::command::HANDSHAKE => {}
                    r#enum::command::REGISTER => {
                        let resp = RegisterResponse::parse_from_bytes(&down.payloadData).unwrap();
                        Arc::get_mut(&mut request).unwrap().register(
                            resp.instanceId,
                            resp.sessKey,
                            resp.sdkOption.lz4CompressionThresholdBytes,
                        );

                        writer
                            .lock()
                            .unwrap()
                            .write(&request.keep_alive_request())
                            .unwrap();
                        writer
                            .lock()
                            .unwrap()
                            .write(&request.enter_room_request())
                            .unwrap();
                    }
                    r#enum::command::UNREGISTER => {
                        break;
                    }
                    r#enum::command::KEEP_ALIVE => {}
                    r#enum::command::PING => {}
                    _ => {
                        log::log!(log::Level::Warn, "Unhandled command: {:?}", down)
                    }
                }
            }
            drop(guard);
        }
        Err(e) => {
            panic!("Failed to connect: {}", e);
        }
    }
}
