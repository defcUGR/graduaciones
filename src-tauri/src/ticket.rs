use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use cuid2::CuidConstructor;
use image::{ImageBuffer, Rgb};
use qrcode::QrCode;

type Encryptor = ecb::Encryptor<aes::Aes128>;
type Decryptor = ecb::Decryptor<aes::Aes128>;

const KEY: &[u8] = "asda".as_bytes();

const CUID_CREATOR: CuidConstructor = CuidConstructor::new();

struct TicketData {
    pub session_id: String,
    pub ticket_id: u32,
    pub email: String,
}

#[tauri::command]
pub fn create_ticket_from_email(email: String) -> String {
    general_purpose::STANDARD.encode(create_ticket_code(TicketData {
        session_id: CUID_CREATOR.with_length(10).create_id(),
        ticket_id: 24,
        email,
    }))
}

fn create_ticket_code(data: TicketData) -> Vec<u8> {
    let s = format!("{}{},{}", data.session_id, data.ticket_id, data.email);

    let enc = Encryptor::new(KEY.into()).encrypt_padded_vec_mut::<Pkcs7>(&s.as_bytes());

    enc
}

fn create_ticket_qrcode(data: TicketData) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let code = create_ticket_code(data);

    let qrcode = QrCode::new(&code).unwrap();
    let image = qrcode
        .render()
        .dark_color(Rgb([21, 225, 36]))
        .quiet_zone(false)
        .build();

    image
}
