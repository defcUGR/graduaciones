use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

fn get_credentials() -> Credentials {
    Credentials::new(
        std::env::var("EMAIL").unwrap(),
        std::env::var("EMAIL_PASSWORD").unwrap(),
    )
}

fn get_mailer() -> SmtpTransport {
    let host = std::env::var("EMAIL_HOST").unwrap();
    let port = std::env::var("EMAIL_PORT").unwrap();

    SmtpTransport::relay(&host)
        .unwrap()
        .credentials(get_credentials())
        .build()
}
