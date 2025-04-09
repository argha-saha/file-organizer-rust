use crate::utils::{get_extension_folder, load_config};
use std::ffi::OsStr;
use std::fs::{self};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Organize files by extension
pub fn organize(path: &str, preview: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml");
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(ext_folder) = get_extension_folder(&entry_path, &config) {
                let move_dir = Path::new(entry_path.as_path()).with_file_name(&ext_folder);
                let move_path = move_dir.join(entry_path.file_name().unwrap());

                // File is in the correct directory
                if entry_path.parent() == Some(&move_dir) {
                    continue;
                }

                println!("{} -> {}", entry_path.display(), move_path.display());

                if !preview {
                    fs::create_dir_all(&move_dir)?;
                    fs::rename(&entry_path, &move_path)?;

                    // Log file moves
                    let mut log_file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("undo.log")?;

                    writeln!(log_file, "{} -> {}", move_path.display(), entry_path.display())?;
                }
            }
        }
    }

    Ok(())
}

/// Recursively remove empty directories
pub fn remove_empty_directories(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(path)?;

    // Iterate through the directories
    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        // Skip hidden directories
        if let Some(name) = entry_path.file_name().and_then(OsStr::to_str) {
            if name.starts_with('.') {
                continue;
            }
        }

        // Make sure entry is a directory
        if entry_path.is_dir() {
            remove_empty_directories(&entry_path)?;

            // Read the directory again after processing subdirectories
            // Check to make sure directory is empty
            if fs::read_dir(&entry_path)?.next().is_none() {
                fs::remove_dir(&entry_path)?;
                println!("{}", format!("Removed empty dir: {}", entry_path.display()));
            }
        }
    }

    Ok(())
}

/// Undo the last file organization
pub fn undo_organization() -> Result<(), Box<dyn std::error::Error>> {
    // Open the log file and read it line by line
    let file = fs::File::open("undo.log")?;
    let reader = BufReader::new(file);

    // Iterate through each line and ignore invalid lines
    for line in reader.lines().flatten() {
        let movement: Vec<&str> = line.split(" -> ").collect();

        if movement.len() == 2 {
            let path_from = Path::new(movement[0]);
            let path_to = Path::new(movement[1]);

            // If file exists, move file back to its original location
            if path_from.exists() {
                fs::create_dir_all(path_to.parent().unwrap())?;
                fs::rename(path_from, path_to)?;

                println!("{} {} -> {}", "Undo:", path_from.display(), path_to.display());
            }
        }
    }

    // Delete the log file
    fs::remove_file("undo.log")?;

    Ok(())
}