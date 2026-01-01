use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::Instant;

fn human_bytes(n: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if n >= GB {
        format!("{:.2} GB", n as f64 / GB as f64)
    } else if n >= MB {
        format!("{:.2} MB", n as f64 / MB as f64)
    } else if n >= KB {
        format!("{:.2} KB", n as f64 / KB as f64)
    } else {
        format!("{} B", n)
    }
}

pub fn file_downloader_blocking() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let url = match args.next() {
        Some(u) => u,
        None => {
            eprintln!("Usage: downloader <url> <output_path>");
            std::process::exit(2);
        }
    };
    let out_path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: downloader <url> <output_path>");
            std::process::exit(2);
        }
    };

    let client = Client::builder().gzip(true).brotli(true).build()?;

    println!("Requesting: {}", url);
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        return Err(format!("HTTP request failed: {}", resp.status()).into());
    }

    let total_size: Option<u64> = resp
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    let mut reader = resp;
    let mut file = File::create(&out_path)?;
    let mut buffer = [0u8; 8 * 1024];
    let mut downloaded: u64 = 0;
    let start = Instant::now();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;

        // progress display
        if let Some(total) = total_size {
            let pct = (downloaded as f64 / total as f64) * 100.0;
            let elapsed = start.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 {
                human_bytes((downloaded as f64 / elapsed) as u64)
            } else {
                "-".to_string()
            };
            print!(
                "\rDownloaded: {}/{} ({:.2}%) - {}/s",
                human_bytes(downloaded),
                human_bytes(total),
                pct,
                rate
            );
        } else {
            let elapsed = start.elapsed().as_secs_f64();
            let rate = if elapsed > 0.0 {
                human_bytes((downloaded as f64 / elapsed) as u64)
            } else {
                "-".to_string()
            };
            print!("\rDownloaded: {} - {}/s", human_bytes(downloaded), rate);
        }
        io::stdout().flush()?;
    }
    let elapsed = start.elapsed();
    println!(
        "\nFinished: wrote {} in {:.2}s",
        human_bytes(downloaded),
        elapsed.as_secs_f64()
    );

    Ok(())
}
