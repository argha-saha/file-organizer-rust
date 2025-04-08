mod organizer;
mod utils;

use clap::Parser;

/// File Organizer
#[derive(Parser, Debug)]
#[command(name = "file-organizer", about = "Organize files by extension")]
struct Args {
    /// Path to Organize
    path: String,
    /// Preview (files won't move)
    #[arg(short, long)]
    preview: bool
}

fn main() {
    let args = Args::parse();

    if let Err(e) = organizer::organize(&args.path, args.preview) {
        eprintln!("Error: {}", e);
    }
}
