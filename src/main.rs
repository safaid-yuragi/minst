mod download;
mod install;
mod parse;
mod set_path;

use clap::Parser;
use dirs_next::config_dir;
use std::io::{self, Write, BufRead};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pub prefix: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut prefix = config_dir()
        .expect("Failed to determine config directory")
        .join("minau")
        .join("bin");

    let mut prefix_string = prefix.to_string_lossy().to_string();

    if let Some(cli_prefix) = cli.prefix {
        let temp: &Path = cli_prefix.as_ref();
        prefix = temp.join("minau").join("bin");
        prefix_string = prefix.to_string_lossy().to_string();
    }

    let prompt = format!(
        r#"This program will:
  1. Create directory: {}
  2. Download and install the minau executable
  3. Add the directory to your PATH environment variable

Do you want to proceed? [y/N]: "#,
        prefix_string
    );

    let input = match prompt_user(&prompt) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1);
        }
    };

    if matches!(input.to_lowercase().as_str(), "yes" | "y") {
        install::install_minau(&prefix_string);
        set_path::set_path(prefix.parent().unwrap());
        println!("\nInstallation complete!");
        println!("Please restart your shell or run the appropriate command to update your PATH.");
    } else {
        println!("Installation cancelled.");
    }
}

#[cfg(not(windows))]
fn prompt_user(prompt: &str) -> io::Result<String> {
    use std::fs::File;
    use std::io::BufReader;

    let tty = File::open("/dev/tty")?;
    let mut reader = BufReader::new(tty);

    let mut tty_out = File::create("/dev/tty")?;
    write!(tty_out, "{}", prompt)?;
    tty_out.flush()?;

    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[cfg(windows)]
fn prompt_user(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
