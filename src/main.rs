use clap::Parser;
use dict8r::process_files;
use dict8r::config::read_dictionary;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use std::path::Path;

/// Command-line interface configuration for Dict8r application.
/// Provides functionality for text replacements in files based on a dictionary file.
#[derive(Parser, Debug)]
#[clap(author = "Brad Kovaluk <bkovaluk@gmail.com>", version = "0.1.0", about, long_about = None)]
struct Cli {
    /// Path to the file or directory where text replacements should be applied.
    #[clap(value_parser)]
    path: String,

    /// Path to the JSON dictionary file that contains text replacement rules.
    #[clap(value_parser)]
    dict: String,

    /// Flag to perform a trial run without making any changes to files.
    #[clap(long)]
    dry_run: bool,

    /// Filters to apply on file extensions when processing directories.
    /// Multiple extensions can be specified separated by commas, e.g., -f txt,md.
    #[clap(short, long, value_name = "EXTENSION", value_delimiter = ',', use_value_delimiter = true)]
    filters: Vec<String>,
}

fn main() {
    // Initialize the logger with the specified log level.
    SimpleLogger::new().with_level(LevelFilter::Info).init().expect("Failed to initialize logger");

    // Parse command line arguments using clap.
    let cli = Cli::parse();

    // Load the dictionary from the specified file path.
    let dictionary = read_dictionary(&cli.dict).expect("Failed to load dictionary");

    // Process files according to the specified path, dictionary, and options.
    if let Err(e) = process_files(Path::new(&cli.path), &dictionary, &cli.filters, cli.dry_run) {
        eprintln!("Error processing files: {}", e);
    }
}
