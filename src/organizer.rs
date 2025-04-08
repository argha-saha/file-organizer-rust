use crate::utils::get_extension_folder;
use std::fs;
use std::path::Path;

pub fn organize(path: &str, preview: bool) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext_folder) = get_extension_folder(&path) {
                let move_dir = Path::new(path.as_path()).with_file_name(&ext_folder);
                let move_path = move_dir.join(path.file_name().unwrap());

                if path.parent() == Some(&move_dir) {
                    continue;
                }

                println!("{} -> {}", path.display(), move_path.display());

                if !preview {
                    fs::create_dir_all(&move_dir)?;
                    fs::rename(&path, &move_path)?;
                }
            }
        }
    }

    Ok(())
}