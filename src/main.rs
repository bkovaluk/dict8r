use clap::Parser;
use dict8r::process_files;
use dict8r::config::read_dictionary;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use std::path::Path;

/// Dictates text replacements in files using a dictionary file.
#[derive(Parser, Debug)]
#[clap(author = "Brad Kovaluk <bkovaluk@gmail.com>", version = "0.1.0", about, long_about = None)]
struct Cli {
    /// The file or directory to process
    #[clap(value_parser)]
    path: String,

    /// The dictionary file
    #[clap(value_parser)]
    dict: String,

    /// Perform a trial run with no changes made
    #[clap(long)]
    dry_run: bool,

    /// Process directories recursively
    #[clap(short, long)]
    recursive: bool,

    /// Filter files by extension, e.g., -f txt -f md
    #[clap(short, long, value_name = "EXTENSION", value_delimiter = ',', use_value_delimiter = true)]
    filters: Vec<String>,
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let cli = Cli::parse();

    let dictionary = read_dictionary(&cli.dict).expect("Failed to load dictionary");

    if let Err(e) = process_files(Path::new(&cli.path), &dictionary, &cli.filters, cli.recursive, cli.dry_run) {
        eprintln!("Error: {}", e);
    }
}
