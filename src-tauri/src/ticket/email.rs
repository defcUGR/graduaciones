use lettre::message::header::ContentType;
use lettre::message::{Attachment, Body, MessageBuilder, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport};

fn get_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    Ok(Credentials::new(
        std::env::var("EMAIL")?,
        std::env::var("EMAIL_PASSWORD")?,
    ))
}

pub fn get_mailer() -> Result<SmtpTransport, Box<dyn std::error::Error>> {
    let host = std::env::var("EMAIL_HOST")?;
    let port = std::env::var("EMAIL_PORT")?;

    Ok(SmtpTransport::relay(&host)?
        .port(port.parse::<u16>()?)
        .tls(Tls::Required(TlsParameters::new(
            "smtp.ugr.es".to_string(),
        )?))
        .credentials(get_credentials()?)
        .build())
}

fn add_extra_mail(msg: MessageBuilder, record: &super::csv::Record) -> MessageBuilder {
    if let Some(extra_mail) = record.extra_mail.as_ref() {
        msg.cc((&extra_mail).parse().unwrap())
    } else {
        msg
    }
}

fn get_body(record: &super::csv::Record) -> String {
    format!(
        r"¡Hola {}!

Se adjuntan en este correo tu entrada para la graduación y las {} entradas de invitados que se te han concedido.
    
IMPORTANTE: Las entradas de invitados las puede usar cualquier persona, pero la tuya es distinta a las demás y no puede ser usada por otra persona que no seas tú.",
        record.name, record.invitations
    )
}

pub fn get_email(record: &super::csv::Record) -> Result<Message, Box<dyn std::error::Error>> {
    let atts_folder = home::home_dir()
        .unwrap()
        .join(".graduaciones")
        .join("codes")
        .join(&record.mail);

    let graduado_filename = record.name.clone() + ".pdf";
    let mut parts = MultiPart::mixed()
        .singlepart(SinglePart::builder().body(get_body(record)))
        .singlepart(Attachment::new(graduado_filename.clone()).body(
            Body::new(std::fs::read(atts_folder.join(&graduado_filename))?),
            ("application/pdf").parse()?,
        ));

    for i in 0..record.invitations {
        let filename = format!("{} - Invitado {}.pdf", record.name.clone(), i + 1);
        parts = parts.singlepart(Attachment::new(filename.clone()).body(
            Body::new(std::fs::read(atts_folder.join(&filename))?),
            ("application/pdf").parse()?,
        ));
    }

    Ok(add_extra_mail(
        Message::builder()
            .from("DEFC <defc@ugr.es>".parse().unwrap())
            .to(format!("{} <{}>", record.name.clone(), record.mail)
                .parse()
                .unwrap())
            .subject("ENTRADAS GRADUACIÓN"),
        record,
    )
    .multipart(parts)?)
}
