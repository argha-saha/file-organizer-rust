use std::path::Path;

pub fn get_extension_folder(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext.to_lowercase().as_str() {
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
        }.to_string())
}