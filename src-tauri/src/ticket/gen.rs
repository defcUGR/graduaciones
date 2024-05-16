use image::{ImageBuffer, Rgb};
use lopdf::{xobject, Document};
use qrcode::QrCode;

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

pub fn create_ticket_pdf(data: TicketData) -> () {
    let base64 = get_base64_code(&data);
    let qr_save_path = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes")
        .join(base64.clone() + ".png");
    let pdf_save_path = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes")
        .join(base64 + ".pdf");

    let mut template =
        Document::load("/home/hipy/dev/defc/graduaciones/src-tauri/src/template.pdf").unwrap();
    let pages = template.get_pages();
    println!("pages: {:?}", pages);
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

    debug!("Created ticket PDF in {:?}", &pdf_save_path);
}

pub fn create_many_tickets_pdf(data: Vec<TicketData>) {
    for d in data {
        create_ticket_pdf(d);
    }
}
