use crate::download;
use crate::parse;
use async_compat::CompatExt;
use reqwest::Client;
use std::env;
use zip::ZipArchive;
use std::path::Path;

const MINAU_RELEASE_API: &str =
    "https://api.github.com/repos/sirasaki-konoha/minau/releases/latest";

pub fn install_minau(prefix: &str) {
    let release = parse_release();
    let client = Client::new();
    let download_to = format!("temp_minau_{}", release.0);

    println!("Installing minau {}...", release.0);

    smol::block_on(async {
        download::download_file(&client, &release.2, &download_to, &format!("downloading {}", release.1))
            .compat()
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to download file: {}", e);
                let _ = std::fs::remove_file(&download_to);
                std::process::exit(1);
            });
    });

    let file = std::fs::File::open(&download_to).unwrap_or_else(|e| {
        eprintln!("Failed to open zip file: {}", e);
        let _ = std::fs::remove_file(&download_to);
        std::process::exit(1);
    });

    let mut archive = ZipArchive::new(file).unwrap_or_else(|e| {
        eprintln!("Failed to read zip archive: {}", e);
        let _ = std::fs::remove_file(&download_to);
        std::process::exit(1);
    });

    let prefix_path: &Path = prefix.as_ref();
    
    if let Err(e) = std::fs::create_dir_all(prefix_path) {
        eprintln!("Failed to create directory {}: {}", prefix_path.display(), e);
        let _ = std::fs::remove_file(&download_to);
        std::process::exit(1);
    }

    let mut found = false;
    for index in 0..archive.len() {
        let mut file = match archive.by_index(index) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to read file from archive: {}", e);
                continue;
            }
        };

        let target_name = if cfg!(windows) { "minau.exe" } else { "minau" };
        
        if file.name() == target_name {
            let output = prefix_path.join(target_name);
            let mut outfile = std::fs::File::create(&output).unwrap_or_else(|e| {
                eprintln!("Failed to create minau executable file: {}", e);
                let _ = std::fs::remove_file(&download_to);
                std::process::exit(1);
            });
            
            if let Err(e) = std::io::copy(&mut file, &mut outfile) {
                eprintln!("Failed to extract minau executable: {}", e);
                let _ = std::fs::remove_file(&download_to);
                std::process::exit(1);
            }
            
            found = true;
            
            #[cfg(not(windows))]
            add_executable(&output);
            
            break;
        } 
    }

    if !found {
        eprintln!("minau executable not found in the archive");
        let _ = std::fs::remove_file(&download_to);
        std::process::exit(1);
    }

    let _ = std::fs::remove_file(&download_to);
    println!("Successfully installed minau to {}", prefix_path.display());
}

pub fn parse_release() -> (String, String, String) {
    let mut release_info = String::new();
    smol::block_on(async {
        release_info = Client::new()
            .get(MINAU_RELEASE_API)
            .header("User-Agent", format!("minst/{}", env!("CARGO_PKG_VERSION")))
            .send()
            .compat()
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to fetch latest release: {}", e);
                std::process::exit(1);
            })
            .text()
            .compat()
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to read response text: {}", e);
                std::process::exit(1);
            });
    });

    let parsed = parse::parse(&release_info);
    let latest = parsed.tag_name;
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let mut found = false;
    let mut name = String::new();
    let mut download_url = String::new();

    let expected_name = if cfg!(windows) {
        format!("minau-{}-{}.exe.zip", os, arch)
    } else {
        format!("minau-{}-{}.zip", os, arch)
    };

    for asset in parsed.assets {
        if asset.name == expected_name {
            name = asset.name;
            download_url = asset.browser_download_url;
            found = true;
            break;
        }
    }
    
    if !found {
        eprintln!("No release found for {} {}", os, arch);
        eprintln!("Expected file: {}", expected_name);
        std::process::exit(1);
    }

    (latest, name, download_url)
}

#[cfg(not(windows))]
fn add_executable(file: &Path) {
    use std::{fs::Permissions, os::unix::fs::PermissionsExt};
    
    let permissions = Permissions::from_mode(0o755);
    
    std::fs::set_permissions(file, permissions).unwrap_or_else(|e| {
        eprintln!("Failed to set executable permission for {}: {}", file.display(), e);
        std::process::exit(1);
    });
}
