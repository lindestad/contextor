// scanner.rs (Multithreaded File Scanning with .gitignore Support)
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
            };
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
    use serial_test::serial;
    use std::fs::File;
    use std::io::Write;

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

    #[test]
    #[serial]
    fn test_gitignore_exclusion() {
        let test_dir = "test_gitignore";
        fs::create_dir_all(test_dir).unwrap();

        // Create a .gitignore file
        let gitignore_path = format!("{}/.gitignore", test_dir);
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "ignored_file.txt").unwrap();

        // Create an ignored file
        let ignored_file_path = format!("{}/ignored_file.txt", test_dir);
        fs::write(&ignored_file_path, "This should be ignored").unwrap();

        // Create a non-ignored file
        let valid_file_path = format!("{}/valid_file.txt", test_dir);
        fs::write(&valid_file_path, "This should be included").unwrap();

        let results = scan_project(test_dir);

        assert!(results.iter().any(|f| f.path.ends_with("valid_file.txt")));
        assert!(!results.iter().any(|f| f.path.ends_with("ignored_file.txt")));

        fs::remove_dir_all(test_dir).unwrap();
    }
}
