use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::path::Path;

fn required_env(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(key).map_err(|_| format!("Missing required environment variable: {key}").into())
}

fn first_present_env(keys: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    for key in keys {
        if let Ok(value) = env::var(key) {
            return Ok(value);
        }
    }

    Err(format!("Missing required environment variable, tried: {}", keys.join(", ")).into())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if let Err(primary_err) = dotenvy::from_path(&env_path) {
        if let Err(fallback_err) = dotenvy::dotenv() {
            eprintln!(
                "Warning: failed to load .env from {} ({primary_err}) and current directory ({fallback_err})",
                env_path.display()
            );
        }
    }

    let smtp_username = first_present_env(&["SMTP_USERNAME", "SMTP_USER"])?;
    let smtp_password = first_present_env(&["SMTP_PASSWORD", "SMTP_PASS"])?;
    let smtp_host = required_env("SMTP_HOST")?;
    let smtp_port: u16 = required_env("SMTP_PORT")?.parse()?;
    let from_name = required_env("FROM_NAME")?;
    let from_email = required_env("FROM_EMAIL")?;
    let to_name = required_env("TO_NAME")?;
    let to_email = required_env("TO_EMAIL")?;

    // Define the email
    let from_mailbox = format!("{from_name} <{from_email}>").parse()?;
    let reply_to_mailbox = from_email.parse()?;
    let to_mailbox = format!("{to_name} <{to_email}>").parse()?;

    let email = Message::builder()
        .from(from_mailbox)
        .reply_to(reply_to_mailbox)
        .to(to_mailbox)
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))?;
    // Set up the SMTP client
    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to SMTP relay host
    let mailer = SmtpTransport::starttls_relay(&smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
