use crate::scraper::Job;
use std::{fs, io, path::Path};

mod email;
mod scraper;

fn vec_difference<T>(current: &[T], previous: &[T]) -> Vec<T>
where
    T: PartialEq + Clone,
{
    current
        .iter()
        .filter(|item| !previous.contains(*item))
        .cloned()
        .collect()
}

fn save_jobs(path: impl AsRef<Path>, jobs: &Vec<Job>) -> io::Result<()> {
    let bytes = bincode::serde::encode_to_vec(jobs, bincode::config::standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, bytes)
}

fn load_jobs(path: impl AsRef<Path>) -> io::Result<Vec<Job>> {
    let bytes = fs::read(path)?;
    let (jobs, _): (Vec<Job>, usize) =
        bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(jobs)
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let previous_jobs = load_jobs("previous_jobs")?;
    let jobs = scraper::get_jobs("https://defencescholarships.aigroup.com.au/placements/")?;

    // println!("Found {} jobs from defence scholarships", jobs.len());
    // for (index, job) in jobs.iter().enumerate() {
    //     println!(
    //         "{}. {} | {} | {} | {} | {} | {}",
    //         index + 1,
    //         job.title,
    //         job.company,
    //         job.location,
    //         job.link,
    //         job.logo_url,
    //         job.posted
    //     );
    // }

    let diff = vec_difference(&jobs, &previous_jobs);

    
    email::send_email(diff)?;

    save_jobs("previous_jobs", &jobs)?;

    Ok(())
}
