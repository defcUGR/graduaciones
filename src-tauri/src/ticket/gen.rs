use std::path::Path;

use image::{ImageBuffer, Rgb};
use lopdf::{xobject, Document};
use qrcode::QrCode;

use crate::ticket::data::AttendantType;

use super::data::{create_ticket_code, get_base64_code, TicketData};

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

pub fn create_ticket_pdf(data: TicketData, pdf_save_path: &dyn AsRef<Path>) -> () {
    let base64 = get_base64_code(&data);
    let qr_tmp_folder = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes")
        .join("tmp_qrs");
    std::fs::create_dir(&qr_tmp_folder);
    let qr_save_path = qr_tmp_folder.join(base64.replace("/", "_") + ".png");

    let mut template = match data.attendant_type {
        AttendantType::Invited => {
            Document::load("/home/hipy/dev/defc/graduaciones/src-tauri/src/template.pdf").unwrap()
        }
        AttendantType::Graduated => {
            Document::load("/home/hipy/dev/defc/graduaciones/src-tauri/src/graduado.pdf").unwrap()
        }
    };
    let pages = template.get_pages();
    println!("pages: {:?}, {:?}", pages, &qr_save_path);
    let first_page = pages.get(&1).unwrap();

    let qr_code = create_ticket_qrcode(data);
    qr_code.save(&qr_save_path).unwrap();

    template
        .insert_image(
            *first_page,
            xobject::image(&qr_save_path).unwrap(),
            (1120.0, 175.0),
            (480.0, 480.0),
        )
        .unwrap();

    template.save(&pdf_save_path).unwrap();

    debug!("Created ticket PDF in {:?}", &pdf_save_path.as_ref());
}
