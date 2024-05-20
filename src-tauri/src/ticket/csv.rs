use std::path::Path;

use csv::Reader;
use lettre::Transport;

use super::data::{AttendantType, TicketData};
use super::email::{get_email, get_mailer};

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub mail: String,
    pub extra_mail: Option<String>,
    pub name: String,
    pub invitations: u32,
    pub not_sent: u8, // 0 | 1
}

pub fn gen_ticket_from_record(
    session_id: String,
    record: &Record,
    folder: &dyn AsRef<Path>,
    id: u32,
    attendant_type: super::data::AttendantType,
) {
    let save_name = if attendant_type == AttendantType::Graduated {
        format!("{}.pdf", record.name)
    } else {
        format!("{} - Invitado.pdf", record.name)
    };

    super::create_ticket_pdf(
        TicketData {
            session_id: session_id.clone(),
            attendant_type,
            ticket_id: id,
            email: record.mail.clone(),
        },
        &folder.as_ref().join(&save_name),
    );
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

        if record.not_sent == 1 {
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

            debug!("Sent {}", record.mail);
        } else {
            debug!("Skipping {} as not sent", record.mail);

            ticket_counter += record.invitations + 1;
        }
    }

    Ok(())
}
