use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// #[tokio::main]
pub async fn blocking_downloader() -> Result<(), Box<dyn Error>> {
    let url = "https://fsn1-speed.hetzner.com/100MB.bin";
    let output_path = "downloaded.bin";

    let client = Client::new();
    println!("Downloading from: {}", url);

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Request failed: {}", response.status()).into());
    }

    // Get total size if available
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    let mut file = File::create(output_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("Download complete!");
    println!("Saved to {}", output_path);

    Ok(())
}
