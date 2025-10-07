use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<ReleaseAsset>,
}

pub fn parse(text: &str) -> Release {
    let parsed: Release = serde_json::from_str(text).unwrap_or_else(|e| {
        eprintln!("Failed to parse github api json\n{}", e);
        std::process::exit(1);
    });

    parsed
}
