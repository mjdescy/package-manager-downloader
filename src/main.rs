use clap::{Parser, Subcommand};

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
    Download {
        source: String,
        destination: String,
    },
}

fn main() {
    let cli = Cli::parse(); // panics on bad input, prints --help/--version
    match cli.command {
        Some(Commands::Download { source, destination }) => {
            println!("Downloading from URL: {} to destination: {}", source, destination);
        }
        None => {}
    }
}