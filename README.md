# Job Scraper

## Description

`job_scraper` is a Rust command-line tool that:

- Scrapes job listings from the Defence Scholarships placements page.
- Compares the latest scrape against a previously saved snapshot.
- Detects newly listed jobs.
- Sends an HTML email summary of new jobs (or a no-new-jobs message).
- Saves the latest job snapshot for the next run.

This project is useful for lightweight job monitoring and daily alerting without running a full web app.

## Features

- HTML scraping with CSS selectors via `scraper`
- Blocking HTTP fetch with `reqwest`
- Snapshot persistence using `bincode` + `serde`
- SMTP email sending with `lettre`
- `.env` support with `dotenvy`

## Requirements

- Rust toolchain (stable)
- Network access to:
	- target jobs page
	- your SMTP server

## Configuration

Create a `.env` file in the project root:

```env
SMTP_USERNAME=your_smtp_username
SMTP_PASSWORD=your_smtp_password
SMTP_HOST=smtp.your-provider.com
SMTP_PORT=587
FROM_NAME=Job Scraper
FROM_EMAIL=sender@example.com
TO_NAME=Your Name
TO_EMAIL=recipient@example.com
```

Notes:

- `SMTP_USER` can be used instead of `SMTP_USERNAME`.
- `SMTP_PASS` can be used instead of `SMTP_PASSWORD`.

## Usage

This app is designed to run as a cron task so it can check for new jobs on a schedule.

### 1) Prepare a runtime folder

Create a folder that contains:

- the compiled binary (for example `job_scraper`)
- your `.env` file
- the `previous_jobs` snapshot file

Example:

```bash
mkdir -p /opt/job_scraper
cp ./target/release/job_scraper /opt/job_scraper/
touch /opt/job_scraper/previous_jobs
```

Make sure the binary is executable:

```bash
chmod +x /opt/job_scraper/job_scraper
```

### 2) Add a cron entry

Open your user crontab:

```bash
crontab -e
```

Add a job line (this example runs every day at 8:00 AM):

```cron
0 8 * * * cd /opt/job_scraper && ./job_scraper >> /opt/job_scraper/cron.log 2>&1
```

### 3) Cron settings explained

Cron format:

```text
┌──────── minute (0-59)
│ ┌────── hour (0-23)
│ │ ┌──── day of month (1-31)
│ │ │ ┌── month (1-12)
│ │ │ │ ┌ day of week (0-7, where 0 and 7 are Sunday)
│ │ │ │ │
* * * * * command
```

### 4) Useful schedule examples

```cron
# Every 30 minutes
*/30 * * * * cd /opt/job_scraper && ./job_scraper >> /opt/job_scraper/cron.log 2>&1

# Weekdays at 9:00 AM
0 9 * * 1-5 cd /opt/job_scraper && ./job_scraper >> /opt/job_scraper/cron.log 2>&1

# Daily at midnight
0 0 * * * cd /opt/job_scraper && ./job_scraper >> /opt/job_scraper/cron.log 2>&1
```

### 5) Verify and monitor

List installed cron jobs:

```bash
crontab -l
```

Watch logs:

```bash
tail -f /opt/job_scraper/cron.log
```


