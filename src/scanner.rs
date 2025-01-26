use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct ScannedFile {
    pub path: String,
    pub content: Option<String>, // None for binary files
    pub is_binary: bool,
}

pub fn scan_project(folder_path: &str, max_file_size: u64) -> Vec<ScannedFile> {
    let walker = WalkBuilder::new(folder_path)
        .git_ignore(true) // Enables .gitignore filtering
        .hidden(false) // Exclude hidden files
        .parents(true) // Respect .gitignore in parent directories
        .build();

    // Collect paths before processing to avoid race conditions
    let files: Vec<PathBuf> = walker
        .filter_map(|entry| {
            if let Ok(e) = entry {
                if !e.file_type()?.is_file() {
                    return None; // Skip directories
                }
                Some(e.path().to_path_buf())
            } else {
                None
            }
        })
        .collect();

    // Process files in parallel using rayon
    files
        .par_iter()
        .filter_map(|path| process_file(path, max_file_size))
        .collect()
}

fn process_file(path: &PathBuf, max_file_size: u64) -> Option<ScannedFile> {
    if let Ok(metadata) = fs::metadata(path) {
        let file_size = metadata.len();

        if file_size > max_file_size {
            return Some(ScannedFile {
                path: path.to_string_lossy().to_string(),
                content: Some(format!(
                    "[File size > {:.1}MB (max: {:.1}MB)]",
                    file_size as f64 / 1_000_000.0,
                    max_file_size as f64 / 1_000_000.0
                )),
                is_binary: false,
            });
        }

        if metadata.is_file() {
            return Some(read_file(path));
        }
    }
    None
}

fn read_file(path: &PathBuf) -> ScannedFile {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => {
            return ScannedFile {
                path: path.to_string_lossy().to_string(),
                content: None,
                is_binary: true,
            };
        }
    };

    let is_binary = is_binary(&data);

    let content = if is_binary {
        None
    } else {
        Some(truncate_text(
            String::from_utf8_lossy(&data).to_string(),
            10_000_000,
        ))
    };

    ScannedFile {
        path: path.to_string_lossy().to_string(),
        content,
        is_binary,
    }
}

fn is_binary(data: &[u8]) -> bool {
    data.iter().any(|&b| b == 0)
}

fn truncate_text(text: String, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}\n[Truncated: File too large]", &text[..max_len as usize])
    } else {
        text
    }
}
