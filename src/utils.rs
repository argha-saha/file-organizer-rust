use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Determines the folder name based on the file extension
pub fn get_extension_folder(path: &Path, config: &HashMap<String, String>) -> Option<String> {
    let ext = path.extension()?.to_str()?.to_lowercase();

    // Check the config file
    if let Some(folder) = config.get(&ext) {
        return Some(folder.clone());
    }

    let folder = match ext.as_str() {
        "pdf" => "PDFs",
        "png" | "jpg" | "jpeg" | "gif" | "tiff" | "bmp" => "Images",
        "mp3" | "wav" | "aac" | "flac" | "aiff" => "Audio",
        "mp4" | "mov" | "avi" => "Videos",
        "zip" | "rar" | "tar" | "gz" | "7z" => "Archives",
        "doc" | "docx" => "Word Docs",
        "txt" | "rtf" | "md" => "Text Docs",
        "xls" | "xlsx" => "Excel Sheets",
        "ppt" | "pptx" => "PowerPoint Slides",
        "csv" | "json" | "xml" => "Data Documents",
        "exe" | "msi" => "Executables",
        "bin" => "Binaries",
        _ => "Misc"
    };

    Some(folder.to_string())
}

/// Load config from a TOML file
pub fn load_config(path: &str) -> HashMap<String, String> {
    let content = fs::read_to_string(path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_default()
}