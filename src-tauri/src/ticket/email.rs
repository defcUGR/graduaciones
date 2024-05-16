use lettre::transport::smtp::authentication::Credentials;

fn get_credentials() -> Credentials {
    Credentials::new(
        std::env::var("EMAIL").unwrap(),
        std::env::var("EMAIL_PASSWORD").unwrap(),
    )
}
