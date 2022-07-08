use super::super::prelude::*;

use crate::DownstreamPayload::DownstreamPayload;
use crate::PacketHeader::packet_header::EncryptionMode;
use crate::PacketHeader::PacketHeader;

const HEADER_OFFSET: usize = 12;

pub fn convert_key(token: &String) -> [u8; 32] {
    base64::decode(token).unwrap().try_into().unwrap()
}

fn encrypt(key: &[u8; 32], body: Vec<u8>) -> Vec<u8> {
    let iv = Aes256CbcEnc::generate_iv(thread_rng());

    let mut encrypted = Aes256CbcEnc::new(key.into(), &iv).encrypt_padded_vec_mut::<Pkcs7>(&body);

    let mut result = iv.to_vec();

    result.append(&mut encrypted);

    result
}

fn decrypt(key: &[u8; 32], body: &[u8]) -> Vec<u8> {
    let iv = &body[..16];

    Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(&body[16..])
        .expect("danmaku decrypt error")
}

pub fn encode(mut header: Vec<u8>, body: Vec<u8>, key: &[u8; 32]) -> Vec<u8> {
    let mut encrypted = encrypt(key, body);

    let header_len = header.len();
    let payload_len = encrypted.len();

    let mut buf = vec![0u8; HEADER_OFFSET + header_len + payload_len];
    buf.push(0xAB);
    buf.push(0xCD);
    buf.push(0x00);
    buf.push(0x01);
    buf.extend(&header_len.to_be_bytes());
    buf.extend(&payload_len.to_be_bytes());
    buf.append(&mut header);
    buf.append(&mut encrypted);

    buf
}

pub fn decode(
    data: &Vec<u8>,
    security_key: &[u8; 32],
    session_key: &[u8; 32],
) -> (PacketHeader, DownstreamPayload) {
    let header_len = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
    let payload_len = u32::from_be_bytes(data[8..12].try_into().unwrap()) as usize;

    let header = PacketHeader::parse_from_bytes(&data[HEADER_OFFSET..header_len])
        .expect("PacketHeader parse error");

    let payload = match header.encryptionMode.unwrap() {
        EncryptionMode::kEncryptionNone => {
            if payload_len != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&data[HEADER_OFFSET + header_len..])
                .expect("DownstreamPayload parse error")
        }
        EncryptionMode::kEncryptionServiceToken => {
            let decrypted = decrypt(&security_key, &data[HEADER_OFFSET + header_len..]);
            if decrypted.len() != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&decrypted).expect("DownstreamPayload parse error")
        }
        EncryptionMode::kEncryptionSessionKey => {
            let decrypted = decrypt(&session_key, &data[HEADER_OFFSET + header_len..]);
            if decrypted.len() != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&decrypted).expect("DownstreamPayload parse error")
        }
    };

    (header, payload)
}
