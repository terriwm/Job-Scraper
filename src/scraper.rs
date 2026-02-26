use reqwest::blocking;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub location: String,
    pub link: String,
    pub logo_url: String,
    pub posted: String,
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

pub fn get_jobs(url: &str) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    let html = get_job_html(url)?;
    let jobs = parse_jobs_from_string(&html);

    Ok(jobs)
}