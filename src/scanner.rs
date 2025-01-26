use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ScannedFile {
    pub path: String, // Now stores relative path
    pub content: Option<String>,
    pub is_binary: bool,
}

pub fn scan_project(folder_path: &str, max_file_size: u64) -> Vec<ScannedFile> {
    // Convert the input folder to a canonical root path
    // (in case of symlinks, different drive letters, etc.)
    let root_path = match PathBuf::from(folder_path).canonicalize() {
        Ok(p) => p,
        Err(_) => PathBuf::from(folder_path), // Fallback if canonicalize fails
    };

    let walker = WalkBuilder::new(&root_path)
        .git_ignore(true) // Respect .gitignore
        .hidden(false) // Show hidden files (set true if you want them excluded)
        .parents(true) // Respect parent directory .gitignore
        .build();

    // Collect paths (files only) before processing
    let files: Vec<PathBuf> = walker
        .filter_map(|entry| {
            if let Ok(e) = entry {
                let path = e.path();
                // If the path component is ".git", skip it
                if path.components().any(|c| c.as_os_str() == ".git") {
                    return None;
                }
                if e.file_type()?.is_file() {
                    // Canonicalize the path so we can strip the root reliably
                    let abs_path = e.path().canonicalize().ok()?;
                    return Some(abs_path);
                }
            }
            None
        })
        .collect();

    // Process files in parallel (rayon)
    files
        .par_iter()
        .filter_map(|abs_path| {
            // Convert absolute path to relative (strip the root)
            let relative = abs_path
                .strip_prefix(&root_path)
                .unwrap_or(abs_path) // fallback if strip_prefix fails
                .to_path_buf();

            process_file(&root_path, &relative, max_file_size)
        })
        .collect()
}

/// This function expects the **root** path plus the **relative** path.
fn process_file(root_path: &Path, relative: &Path, max_file_size: u64) -> Option<ScannedFile> {
    // Reconstruct the absolute path for reading
    let full_path = root_path.join(relative);

    if let Ok(metadata) = fs::metadata(&full_path) {
        let file_size = metadata.len();

        // If the file is too large, just store a placeholder
        if file_size > max_file_size {
            return Some(ScannedFile {
                path: relative.to_string_lossy().to_string(), // store relative
                content: Some(format!(
                    "[File size > {:.1}MB (max: {:.1}MB)]",
                    file_size as f64 / 1_000_000.0,
                    max_file_size as f64 / 1_000_000.0
                )),
                is_binary: false,
            });
        }

        // Otherwise, read the file content (or detect if it's binary)
        let data = fs::read(&full_path).unwrap_or_default();
        let is_bin = is_binary(&data);

        let content = if is_bin {
            None
        } else {
            Some(truncate_text(
                String::from_utf8_lossy(&data).to_string(),
                10_000_000,
            ))
        };

        return Some(ScannedFile {
            path: relative.to_string_lossy().to_string(),
            content,
            is_binary: is_bin,
        });
    }

    None
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
