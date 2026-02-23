use lettre::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_host = env::var("SMTP_HOST")?;
    let from_name = env::var("FROM_NAME")?;
    let from_email = env::var("FROM_EMAIL")?;
    let to_name = env::var("TO_NAME")?;
    let to_email = env::var("TO_EMAIL")?;

    // Define the email
    let from_mailbox = format!("{from_name} <{from_email}>").parse()?;
    let reply_to_mailbox = from_email.parse()?;
    let to_mailbox = format!("{to_name} <{to_email}>").parse()?;

    let email = Message::builder()
        .from(from_mailbox)
        .reply_to(reply_to_mailbox)
        .to(to_mailbox)
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();
    // Set up the SMTP client
    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to SMTP relay host
    let mailer = SmtpTransport::relay(&smtp_host)?.credentials(creds).build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
