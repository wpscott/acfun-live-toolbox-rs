use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use protobuf::Message;
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

use crate::DownstreamPayload::DownstreamPayload;
use crate::PacketHeader::packet_header::EncryptionMode;
use crate::PacketHeader::PacketHeader;

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

const HEADER_OFFSET: usize = 12;

pub fn convert_key(token: String) -> [u8; 32] {
    let mut key = [0; 32];
    base64::decode_config_slice(token, base64::STANDARD, &mut key).unwrap();
    key
}

fn encrypt(key: &[u8; 32], body: Vec<u8>) -> Vec<u8> {
    let iv = Aes256CbcEnc::generate_iv(ChaChaRng::from_entropy());

    let mut encrypted = Aes256CbcEnc::new(key.into(), &iv).encrypt_padded_vec_mut::<Pkcs7>(&body);

    let mut result = iv.to_vec();

    result.append(&mut encrypted);

    result
}

fn decrypt(key: &[u8; 32], body: &[u8]) -> Vec<u8> {
    let iv = &body[0..16];

    Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(&body[16..])
        .unwrap()
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
    buf.extend_from_slice(&header_len.to_be_bytes());
    buf.extend_from_slice(&payload_len.to_be_bytes());
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

    let header = PacketHeader::parse_from_bytes(&data[HEADER_OFFSET..header_len]).unwrap();

    let payload = match header.encryptionMode.unwrap() {
        EncryptionMode::kEncryptionNone => {
            if payload_len != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&data[HEADER_OFFSET + header_len..]).unwrap()
        }
        EncryptionMode::kEncryptionServiceToken => {
            let decrypted = decrypt(&security_key, &data[HEADER_OFFSET + header_len..]);
            if decrypted.len() != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&decrypted).unwrap()
        }
        EncryptionMode::kEncryptionSessionKey => {
            let decrypted = decrypt(&session_key, &data[HEADER_OFFSET + header_len..]);
            if decrypted.len() != (header.decodedPayloadLen as usize) {
                panic!("Invalid payload length")
            }
            DownstreamPayload::parse_from_bytes(&decrypted).unwrap()
        }
    };

    (header, payload)
}
