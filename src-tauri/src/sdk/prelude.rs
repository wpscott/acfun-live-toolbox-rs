pub use super::db;
pub(crate) use super::live::StartPushData;
pub(crate) use super::{Token, User, VERSION};
pub(crate) use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
pub(crate) use aes::Aes256;
pub(crate) use flate2::read::GzDecoder;
pub(crate) use hmac::{Hmac, Mac};
pub(crate) use hyper::{
    body::{aggregate, Buf},
    Body, Client as HyperClient, Method, Request,
};
pub(crate) use hyper_tls::HttpsConnector;
pub use log::LevelFilter;
pub(crate) use protobuf::{EnumOrUnknown, Message, MessageField};
pub(crate) use rand::prelude::*;
pub(crate) use rusqlite::{params, Connection, Result};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use serde_json;
pub(crate) use sha2::Sha256;


pub(crate) use std::io::{Read, Write};
pub(crate) use std::time::Duration;
pub(crate) use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicI64, Ordering},
    sync::{Arc},
    time::{SystemTime, UNIX_EPOCH},
};
pub use tauri::{AppHandle, Manager, State, Window};
pub use tauri_plugin_log::{LogTarget, LoggerBuilder, RotationStrategy};
pub(crate) use tokio::{
    io::AsyncBufReadExt, io::AsyncWriteExt,
    net::TcpStream as TokioTcpStream, sync::Notify, sync::RwLock, time::interval_at,
};
pub(crate) use uuid::fmt::Hyphenated;
pub use uuid::Uuid;

pub(crate) use crate::{
    AppInfo::AppInfo,
    CommonActionSignalComment::CommonActionSignalComment,
    CommonActionSignalGift::CommonActionSignalGift,
    CommonActionSignalLike::CommonActionSignalLike,
    CommonActionSignalUserEnterRoom::CommonActionSignalUserEnterRoom,
    CommonActionSignalUserFollowAuthor::CommonActionSignalUserFollowAuthor,
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
    ZtLiveScActionSignal::ZtLiveScActionSignal,
    ZtLiveScMessage::{zt_live_sc_message::CompressionType, ZtLiveScMessage},
    ZtLiveScStatusChanged::{zt_live_sc_status_changed::Type, ZtLiveScStatusChanged},
};

pub(crate) type NetworkError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Aes256CbcEnc = cbc::Encryptor<Aes256>;
pub(crate) type Aes256CbcDec = cbc::Decryptor<Aes256>;
pub(crate) type HmacSha256 = Hmac<Sha256>;

pub type DidState = Hyphenated;
pub type UserState = RwLock<Option<User>>;
pub type TokenState = RwLock<Option<Token>>;
pub type ClientState = RwLock<Option<Arc<Notify>>>;
