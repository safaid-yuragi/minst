use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;

pub async fn download_file(
    client: &Client,
    url: &str,
    path: &str,
    msg: &str,
) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to GET from '{}': {}", url, e))?;
    
    let total_size = res
        .content_length()
        .ok_or_else(|| format!("Failed to get content length from '{}'", url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/green}] {bytes}/{total_bytes} ({eta})")
            .map_err(|e| format!("Failed to set progress bar template: {}", e))?
            .progress_chars("=>-"),
    );
    pb.set_message(msg.to_string());

    let mut file = File::create(path)
        .map_err(|e| format!("Failed to create file '{}': {}", path, e))?;
    
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result
            .map_err(|e| format!("Error while downloading file: {}", e))?;
        
        file.write_all(&chunk)
            .map_err(|e| format!("Error while writing to file '{}': {}", path, e))?;
        
        downloaded = min(downloaded + chunk.len() as u64, total_size);
        pb.set_position(downloaded);
    }

    pb.finish_with_message(format!("{}: Complete", msg));
    Ok(())
}
