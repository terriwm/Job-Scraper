mod email;
mod scraper;




fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let jobs = scraper::get_jobs("https://defencescholarships.aigroup.com.au/placements/")?;

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

    email::send_email(html_content)?;

    Ok(())
}
