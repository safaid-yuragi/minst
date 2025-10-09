use std::path::Path;
#[cfg(not(windows))]
use std::io::Write;
#[cfg(not(windows))]
use std::fs::OpenOptions;

#[cfg(not(windows))]
pub fn set_path(path: &Path) {
    println!(">> Adding to PATH...");
    let home = dirs_next::home_dir().unwrap();
    let config = dirs_next::config_dir().unwrap();

    let export_for_posix = format!("\n# for minau\nexport PATH=\"{}:$PATH\"\n", path.join("bin").display());
    let export_for_fish = format!("\nfish_add_path {}\n", path.join("bin").display());
    let export_for_nu = format!("\n$env.PATH = ($env.PATH | prepend \"{}\")\n", path.join("bin").display());
    
    let posix_shell_rcs = &[home.join(".bashrc"), home.join(".zshrc"), home.join(".shrc")];
    let fish_config = home.join(".config").join("fish").join("config.fish");
    let nu_config = config.join("nushell").join("config.nu");
    
    for rc in posix_shell_rcs {
        if rc.exists() {
            println!("> {}", rc.display());
            write_to_file(rc, &export_for_posix);
        }
    }
    
    if fish_config.exists() {
        println!("> {}", fish_config.display());
        write_to_file(&fish_config, &export_for_fish);
    }
    
    if nu_config.exists() {
        println!("> {}", nu_config.display());
        write_to_file(&nu_config, &export_for_nu);
    }
}

#[cfg(not(windows))]
fn write_to_file(path: &Path, content: &str) {
    // ファイルを追記モードで開く
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", path.display(), e);
            return;
        }
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        eprintln!("Failed to write export command to {}: {}", path.display(), e);
    }
}

#[cfg(windows)]
pub fn set_path(path: &Path) {
    println!(">> Adding to PATH...");
    use winreg::enums::*;
    use winreg::RegKey;

    let new_path = path.join("bin").to_string_lossy().to_string();
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = match hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to open registry key: {}", e);
            return;
        }
    };
    
    let current_path: String = env.get_value("PATH").unwrap_or_default();

    let updated_path = if current_path.contains(&new_path) {
        println!("PATH already contains {}", new_path);
        return;
    } else {
        if current_path.is_empty() {
            new_path
        } else {
            format!("{};{}", current_path, new_path)
        }
    };

    if let Err(e) = env.set_value("PATH", &updated_path) {
        eprintln!("Failed to update PATH: {}", e);
    } else {
        println!("Successfully added to PATH");
    }
}

