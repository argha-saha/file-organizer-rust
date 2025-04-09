mod organizer;
mod utils;

use clap::Parser;
use std::path::Path;

/// File Organizer
#[derive(Parser, Debug)]
#[command(name = "file-organizer", about = "Organize files by extension")]
struct Args {
    /// Path to Organize
    path: String,

    /// Preview (files won't move)
    #[arg(long)]
    preview: bool,

    /// Undo organization
    #[arg(long)]
    undo: bool,

    /// Only follow rules in the configuration file
    #[arg(long)]
    config_only: bool,

    /// Display the number of files that would be moved and files per folder
    #[arg(long)]
    report: bool,
}

fn main() {
    let args = Args::parse();

    if args.undo {
        if let Err(e) = organizer::undo_organization() {
            eprintln!("Error: {}", e);
        }

        return;
    }

    if let Err(e) = organizer::organize(&args.path, args.preview, args.config_only, args.report) {
        eprintln!("Error: {}", e);
    } else if !args.preview {
        // Organize the files when preview is disabled
        if let Err(e) = organizer::remove_empty_directories(Path::new(&args.path)) {
            eprintln!("Error: {}", e);
        }
    }
}
