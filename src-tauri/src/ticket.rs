use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};
use cuid2::CuidConstructor;

type Encryptor = ecb::Encryptor<aes::Aes128>;
type Decryptor = ecb::Decryptor<aes::Aes128>;

const KEY: &[u8]  = "asda".as_bytes();

const cuid_creator: CuidConstructor = CuidConstructor::new();

struct TicketData {
  pub session_id: String,
  pub ticket_id: u32,
  pub email: String,
}

#[tauri::command]
pub fn create_ticket_from_email(email: String) -> String {
  create_ticket_code(TicketData {
    session_id: cuid_creator.with_length(10).create_id(),
    ticket_id: 24,
    email
  })
}

fn create_ticket_code(data: TicketData) -> String {
  let s = format!("{}{},{}", data.session_id, data.ticket_id, data.email);

  let enc = Encryptor::new(KEY.into()).encrypt_padded_vec_mut::<Pkcs7>(&s.as_bytes());

  std::str::from_utf8(&enc).unwrap().to_owned()
}