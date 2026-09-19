use clap::{Parser, Subcommand};
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
#[command(name = "downloader", version, about = "A modern CLI example")]
struct Cli {
    #[arg(short, long, default_value_t)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Download { source: String, destination: String },
}

async fn download(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let contents = response.bytes().await?;

    let mut file = File::create(path).await?;
    file.write_all(&contents).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse(); // panics on bad input, prints --help/--version
    match cli.command {
        Some(Commands::Download {
            source,
            destination,
        }) => {
            if cli.verbose {
                println!(
                    "Downloading from URL: {} to destination: {}",
                    source, destination
                );
            }
            download(&source, &destination).await?;
            if cli.verbose {
                println!("Download completed successfully.");
            }
        }
        None => {}
    }

    Ok(())
}
