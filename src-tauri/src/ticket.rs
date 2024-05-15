use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use cuid2::CuidConstructor;
use image::{ImageBuffer, Rgb};
use lopdf::{xobject, Document, Stream};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};

type Encryptor = ecb::Encryptor<aes::Aes128>;
type Decryptor = ecb::Decryptor<aes::Aes128>;

const KEY: &[u8] = "7xi8NYe7eW9mF2G*".as_bytes();

const CUID_CREATOR: CuidConstructor = CuidConstructor::new();

#[allow(non_upper_case_globals)]
const mm2pt: f32 = 2.834;

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketData {
    pub session_id: String,
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
        ticket_id: 24,
        email,
    }
}

fn create_ticket_code(data: &TicketData) -> Vec<u8> {
    let s = format!("{}{},{}", data.session_id, data.ticket_id, data.email);

    let encriptor = Encryptor::new_from_slice(KEY).unwrap();
    let enc = encriptor.encrypt_padded_vec_mut::<Pkcs7>(&s.as_bytes());

    enc
}

fn create_ticket_qrcode(data: TicketData) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let code = create_ticket_code(&data);

    let qrcode = QrCode::new(&code).unwrap();
    let image = qrcode
        .render()
        .dark_color(Rgb([21, 24, 33]))
        .quiet_zone(false)
        .build();

    image
}

pub fn create_ticket_pdf(data: TicketData) -> () {
    let base64 = get_base64_code(&data);
    let save_path = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes")
        .join(base64.clone() + ".png");

    let mut template =
        Document::load("/home/hipy/dev/defc/graduaciones/src-tauri/src/template.pdf").unwrap();
    let pages = template.get_pages();
    println!("pages: {:?}", pages);
    let first_page = pages.get(&1).unwrap();

    let qr_code = create_ticket_qrcode(data);
    qr_code.save(&save_path).unwrap();

    template
        .insert_image(
            *first_page,
            xobject::image(&save_path).unwrap(),
            (1120.0, 175.0),
            (480.0, 480.0),
        )
        .unwrap();

    template
        .save(
            home::home_dir()
                .unwrap()
                .join(".graduaciones")
                .join("codes")
                .join(base64 + ".pdf"),
        )
        .unwrap();
}

pub fn read_data_from_bytes(bytes: &[u8]) -> TicketData {
    let decryptor = Decryptor::new_from_slice(KEY).unwrap();
    let dec = decryptor.decrypt_padded_vec_mut::<Pkcs7>(bytes).unwrap();

    let s = String::from_utf8(dec).unwrap();

    let session_id = &s[0..10];
    let parts = &s[10..].split_once(',').unwrap();
    let ticket_id = parts.0.parse::<u32>().unwrap();
    let email = parts.1.to_string();

    TicketData {
        session_id: session_id.to_string(),
        ticket_id,
        email,
    }
}
