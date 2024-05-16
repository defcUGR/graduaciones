use std::path::Path;

use csv::Reader;
use lettre::Transport;

use super::data::TicketData;
use super::email::{get_email, get_mailer};

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub mail: String,
    pub extra_mail: Option<String>,
    pub name: String,
    pub invitations: u32,
}

pub fn send_from_csv(session_id: String, path: &dyn AsRef<Path>) -> Result<(), String> {
    let base_folder = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes");

    let mut rdr = Reader::from_path(path).map_err(|e| e.to_string())?;

    let mut ticket_counter = 1;

    let mailer = get_mailer().map_err(|e| e.to_string())?;

    for result in rdr.deserialize() {
        let record: Record = result.map_err(|e| e.to_string())?;

        let this_folder = base_folder.join(&record.mail);
        std::fs::create_dir_all(&this_folder).map_err(|e| e.to_string())?;

        if this_folder.join(".done").exists() {
            continue;
        };

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

        mailer
            .send(&get_email(&record).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

        std::fs::File::create(this_folder.join(".done")).map_err(|e| e.to_string())?;
    }

    Ok(())
}
