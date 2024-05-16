use std::path::Path;

use csv::Reader;

use super::data::TicketData;

#[derive(Debug, serde::Deserialize)]
struct Record {
    mail: String,
    extra_mail: Option<String>,
    name: String,
    invitations: u32,
}

pub fn send_from_csv(session_id: String, path: &dyn AsRef<Path>) -> Result<(), String> {
    let base_folder = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes");

    let mut rdr = Reader::from_path(path).map_err(|e| e.to_string())?;

    let mut ticket_counter = 1;

    for result in rdr.deserialize() {
        let record: Record = result.map_err(|e| e.to_string())?;

        let this_folder = base_folder.join(&record.mail);
        std::fs::create_dir_all(&this_folder).map_err(|e| e.to_string())?;

        super::create_ticket_pdf(
            TicketData {
                session_id: session_id.clone(),
                attendant_type: super::data::AttendantType::Graduated,
                ticket_id: ticket_counter,
                email: record.mail.clone(),
            },
            &this_folder.join(format!("{}.pdf", record.name)),
        );
        ticket_counter += 1;

        for i in 0..record.invitations {
            super::create_ticket_pdf(
                TicketData {
                    session_id: session_id.clone(),
                    attendant_type: super::data::AttendantType::Invited,
                    ticket_id: ticket_counter,
                    email: record.mail.clone(),
                },
                &this_folder.join(format!("{} - Invitado {}.pdf", record.name, i + 1)),
            );
            ticket_counter += 1;
        }
    }

    Ok(())
}
