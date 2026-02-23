use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::blocking;
use scraper::{Html, Selector};
use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
struct Job {
    title: String,
    company: String,
    location: String,
    link: String,
    logo_url: String,
    posted: String,
}

fn selector(query: &str) -> Selector {
    Selector::parse(query).expect("selector should be valid")
}

fn first_text(element: scraper::ElementRef<'_>, selector: &Selector) -> String {
    element
        .select(selector)
        .next()
        .map(|node| node.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn first_attr(element: scraper::ElementRef<'_>, selector: &Selector, attr: &str) -> String {
    element
        .select(selector)
        .next()
        .and_then(|node| node.value().attr(attr))
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn parse_jobs_from_string(html: &str) -> Vec<Job> {
    let document = Html::parse_document(html);

    let job_selector = selector("ul.job_listings li.job_listing");
    let logo_block_selector = selector("div.job-listing-company-logo");
    let title_selector = selector("h3.job-listing-loop-job__title");
    let company_selector = selector("div.job-listing-company.company strong");
    let location_selector = selector("div.job-location.location");
    let link_selector = selector("a");
    let logo_img_selector = selector("div.job-listing-company-logo img");
    let posted_selector = selector("span.job-published-date time");

    document
        .select(&job_selector)
        .filter(|job| job.select(&logo_block_selector).next().is_some())
        .map(|job| Job {
            title: first_text(job, &title_selector),
            company: first_text(job, &company_selector),
            location: first_text(job, &location_selector),
            link: first_attr(job, &link_selector, "href"),
            logo_url: first_attr(job, &logo_img_selector, "src"),
            posted: first_text(job, &posted_selector),
        })
        .collect()
}

fn get_job_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = blocking::get(url)?.text()?;

    Ok(body)
}

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

fn send_email(body: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    

    let html = get_job_html("https://defencescholarships.aigroup.com.au/placements/")?;
    let jobs = parse_jobs_from_string(&html);

    println!("Found {} jobs from defence scholarships", jobs.len());
    for (index, job) in jobs.iter().enumerate() {
        println!(
            "{}. {} | {} | {} | {} | {} | {}",
            index + 1,
            job.title,
            job.company,
            job.location,
            job.link,
            job.logo_url,
            job.posted
        );
    }
    
    let html_content = r#"
        <html>
            <body>
                <h1>Hello!</h1>
                <p>This is a <strong>test email</strong> from Rust!</p>
            </body>
        </html>
    "#;

    send_email(html_content);

    Ok(())
}
