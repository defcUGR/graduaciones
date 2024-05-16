use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use cuid2::CuidConstructor;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

type Encryptor = ecb::Encryptor<aes::Aes128>;
type Decryptor = ecb::Decryptor<aes::Aes128>;

const KEY: &[u8] = "7xi8NYe7eW9mF2G*".as_bytes();

const CUID_CREATOR: CuidConstructor = CuidConstructor::new();

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Copy)]
#[repr(u8)]
pub enum AttendantType {
    Graduated = 0,
    Invited,
}

impl TryFrom<u8> for AttendantType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AttendantType::Graduated),
            1 => Ok(AttendantType::Invited),
            _ => Err(()),
        }
    }
}

impl Into<u8> for AttendantType {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TicketData {
    pub session_id: String,
    pub attendant_type: AttendantType,
    pub ticket_id: u32,
    pub email: String,
}

pub fn get_base64_code(data: &TicketData) -> String {
    let code = create_ticket_code(data);

    let enc = general_purpose::STANDARD.encode(code);

    enc
}

pub fn create_ticket_data_from_email(email: String) -> TicketData {
    TicketData {
        session_id: CUID_CREATOR.with_length(10).create_id(),
        attendant_type: AttendantType::Invited,
        ticket_id: 24,
        email,
    }
}

pub(crate) fn create_ticket_code(data: &TicketData) -> Vec<u8> {
    let s = format!(
        "{}{},{},{}",
        data.session_id,
        (Into::<u8>::into(&data.attendant_type)),
        data.ticket_id,
        data.email
    );

    let encriptor = Encryptor::new_from_slice(KEY).unwrap();
    let enc = encriptor.encrypt_padded_vec_mut::<Pkcs7>(&s.as_bytes());

    enc
}

pub fn read_data_from_bytes(bytes: &[u8]) -> TicketData {
    let decryptor = Decryptor::new_from_slice(KEY).unwrap();
    let dec = decryptor.decrypt_padded_vec_mut::<Pkcs7>(bytes).unwrap();

    let s = String::from_utf8(dec).unwrap();

    let session_id = &s[0..10];
    let parts = &s[10..].split_once(',').unwrap();
    let ticket_id = parts.0.parse::<u32>().unwrap();

    let parts_2 = parts.1.split_once(',').unwrap();
    let attendant_type: AttendantType = parts_2.0.parse::<u8>().unwrap().try_into().unwrap();
    let email = parts_2.1.to_string();

    TicketData {
        session_id: session_id.to_string(),
        attendant_type,
        ticket_id,
        email,
    }
}
