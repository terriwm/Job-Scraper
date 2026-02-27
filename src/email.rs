use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use crate::scraper::Job;
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

pub fn send_email(jobs: Vec<Job>) -> Result<(), Box<dyn std::error::Error>> {
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

    let body = generate_html(&jobs);

    let email = Message::builder()
        .from(from_mailbox)
        .reply_to(reply_to_mailbox)
        .to(to_mailbox)
        .subject("Job Report")
        .multipart(
            MultiPart::alternative().singlepart(SinglePart::html(body.to_string()))
        )?;
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

fn generate_html(jobs: &[Job]) -> String {
    if jobs.is_empty() {
        return r#"
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Job Update</title>
            </head>
            <body style="font-family: Arial, sans-serif;">
                <h2>There are no new jobs today</h2>
            </body>
        </html>
        "#
        .to_string();
    }

    let cards = jobs
        .iter()
        .map(|job| {
            format!(
                r#"
                <article class="job-card">
                    <img
                        class="job-logo"
                        src="{logo_url}"
                        alt="{company} logo"
                    />
                    <div>
                        <h2 class="job-title">
                            <a href="{link}">{title}</a>
                        </h2>
                        <p class="job-meta">{company} • {location}</p>
                        <p class="job-time">{posted}</p>
                    </div>
                </article>
                "#,
                title = job.title,
                company = job.company,
                location = job.location,
                link = job.link,
                logo_url = job.logo_url,
                posted = job.posted,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let count = jobs.len();
    let plural = if count == 1 { "" } else { "s" };

    format!(
        r#"
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Job Update</title>
                <style>
                    .job-card {{
                        display: flex;
                        gap: 12px;
                        align-items: flex-start;
                        max-width: 720px;
                        padding: 12px;
                        border: 1px solid #ddd;
                        border-radius: 10px;
                        font-family: Arial, sans-serif;
                        margin-bottom: 12px;
                    }}

                    .job-logo {{
                        width: 56px;
                        height: 56px;
                        object-fit: contain;
                        border: 1px solid #eee;
                        border-radius: 8px;
                        background: #fff;
                        flex-shrink: 0;
                    }}

                    .job-title {{
                        margin: 0 0 6px;
                        font-size: 1.05rem;
                    }}

                    .job-meta {{
                        margin: 0;
                        color: #555;
                        font-size: 0.95rem;
                    }}

                    .job-time {{
                        margin-top: 6px;
                        color: #777;
                        font-size: 0.85rem;
                    }}
                </style>
            </head>
            <body>
                <h2 class="job-title">Good Morning, there are {count} new job{plural}:</h2>
                {cards}
            </body>
        </html>
        "#
    )

}