use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct ScannedFile {
    pub path: String,
    pub content: Option<String>, // None for binary files
    pub is_binary: bool,
}

pub fn scan_project(folder_path: &str) -> Vec<ScannedFile> {
    let walker = WalkBuilder::new(folder_path).git_ignore(true).build();

    // Collect paths before processing to avoid race conditions
    let files: Vec<PathBuf> = walker
        .filter_map(|entry| entry.ok().map(|e| e.path().to_path_buf()))
        .collect();

    // Process files in parallel using rayon
    files
        .par_iter()
        .filter_map(|path| process_file(path))
        .collect()
}

fn process_file(path: &PathBuf) -> Option<ScannedFile> {
    if let Ok(metadata) = fs::metadata(path) {
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
            }
        }
    };

    let is_binary = is_binary(&data);

    let content = if is_binary {
        None
    } else {
        Some(truncate_text(
            String::from_utf8_lossy(&data).to_string(),
            10_000,
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
        format!("{}\n[Truncated: File too large]", &text[..max_len])
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_binary() {
        let text_data = b"Hello, world!";
        let binary_data = b"\x00\xFF\xA0\x45";

        assert!(!is_binary(text_data));
        assert!(is_binary(binary_data));
    }

    #[test]
    fn test_truncate_text() {
        let text = "Hello, this is a long text.".to_string();
        let truncated = truncate_text(text.clone(), 10);
        assert_eq!(truncated, "Hello, thi\n[Truncated: File too large]");
    }
}
