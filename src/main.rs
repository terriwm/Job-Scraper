use crate::scraper::Job;
use std::{fs, io, path::Path};

// mod email;
mod scraper;


fn _save_jobs(path: impl AsRef<Path>, jobs: &Vec<Job>) -> io::Result<()> {
    let bytes = bincode::serde::encode_to_vec(jobs, bincode::config::standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, bytes)
}

fn _load_jobs(path: impl AsRef<Path>) -> io::Result<Vec<Job>> {
    let bytes = fs::read(path)?;
    let (jobs, _): (Vec<Job>, usize) =
        bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(jobs)
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let jobs = scraper::get_jobs("https://defencescholarships.aigroup.com.au/placements/")?;

    let temp_job: Vec<Job> = vec![
        Job {
            title: "Software Engineer".to_string(),
            company: "Tech Corp".to_string(),
            location: "Sydney".to_string(),
            link: "https://example.com/job1".to_string(),
            logo_url: "https://example.com/logo1.png".to_string(),
            posted: "2 days ago".to_string(),
        },
        Job {
            title: "Data Analyst".to_string(),
            company: "Data Solutions".to_string(),
            location: "Melbourne".to_string(),
            link: "https://example.com/job2".to_string(),
            logo_url: "https://example.com/logo2.png".to_string(),
            posted: "1 week ago".to_string(),
        },
    ];

    let temp_job2: Vec<Job> = vec![
        Job {
            title: "Data Analyst".to_string(),
            company: "Data Solutions".to_string(),
            location: "Melbourne".to_string(),
            link: "https://example.com/job2".to_string(),
            logo_url: "https://example.com/logo2.png".to_string(),
            posted: "1 week ago".to_string(),
        },
        Job {
            title: "Dog Walker".to_string(),
            company: "Dog Walking Co".to_string(),
            location: "Melbourne".to_string(),
            link: "https://example.com/job2".to_string(),
            logo_url: "https://example.com/logo2.png".to_string(),
            posted: "2 weeks ago".to_string(),
        },
        Job {
            title: "Project Manager".to_string(),
            company: "Management Inc".to_string(),
            location: "Brisbane".to_string(),
            link: "https://example.com/job3".to_string(),
            logo_url: "https://example.com/logo3.png".to_string(),
            posted: "1 month ago".to_string(),
        },
    ];

    let diff: Vec<_> = temp_job2.into_iter().filter(|item| !temp_job.contains(item)).collect();

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
    
    let _html_content = r#"
        <html>
            <body>
                <h1>Hello!</h1>
                <p>This is a <strong>test email</strong> from Rust!</p>
            </body>
        </html>
    "#;

    // email::send_email(html_content)?;

    Ok(())
}
