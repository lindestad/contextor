use contextor::formatter::{build_tree, format_file_contents, format_project_summary};
use contextor::scanner::ScannedFile;
use std::collections::HashMap;

#[test]
fn test_build_tree() {
    let files = vec![
        ScannedFile {
            path: "src/main.rs".to_string(),
            content: Some("fn main() {}".to_string()),
            is_binary: false,
        },
        ScannedFile {
            path: "src/app.rs".to_string(),
            content: Some("pub struct App {}".to_string()),
            is_binary: false,
        },
        ScannedFile {
            path: "assets/logo.png".to_string(),
            content: None,
            is_binary: true,
        },
    ];

    let result = build_tree(&files);

    let expected = vec![
        ("├── assets".to_string(), "assets".to_string()),
        (
            "│   └── logo.png".to_string(),
            "assets/logo.png".to_string(),
        ),
        ("└── src".to_string(), "src".to_string()),
        ("    ├── app.rs".to_string(), "src/app.rs".to_string()),
        ("    └── main.rs".to_string(), "src/main.rs".to_string()),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_format_file_contents() {
    let mut files = vec![
        ScannedFile {
            path: "src/main.rs".to_string(),
            content: Some("fn main() {}".to_string()),
            is_binary: false,
        },
        ScannedFile {
            path: "assets/logo.png".to_string(),
            content: None,
            is_binary: true,
        },
        ScannedFile {
            path: "README.md".to_string(),
            content: None,
            is_binary: false,
        },
    ];

    // Ensure input is sorted before passing to format_file_contents
    files.sort_by(|a, b| a.path.cmp(&b.path));

    let result = format_file_contents(&files);

    let expected = HashMap::from([
        (
            "assets/logo.png".to_string(),
            "assets/logo.png:\n[Binary file]".to_string(),
        ),
        (
            "README.md".to_string(),
            "README.md:\n[Empty file]".to_string(),
        ),
        (
            "src/main.rs".to_string(),
            "src/main.rs:\nfn main() {}".to_string(),
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_format_project_summary() {
    let tree = vec![
        ("├── assets".to_string(), "assets".to_string()),
        (
            "│   ├── logo.png".to_string(),
            "assets/logo.png".to_string(),
        ),
        ("└── src".to_string(), "src".to_string()),
        ("    ├── app.rs".to_string(), "src/app.rs".to_string()),
        ("    ├── main.rs".to_string(), "src/main.rs".to_string()),
    ];

    let file_contents = HashMap::from([
        (
            "assets/logo.png".to_string(),
            "assets/logo.png:\n[Binary file]".to_string(),
        ),
        (
            "src/app.rs".to_string(),
            "src/app.rs:\npub struct App {}".to_string(),
        ),
        (
            "src/main.rs".to_string(),
            "src/main.rs:\nfn main() {}".to_string(),
        ),
    ]);

    let expected_output = "\
├── assets
│   ├── logo.png
└── src
    ├── app.rs
    ├── main.rs

assets/logo.png:
[Binary file]

src/app.rs:
pub struct App {}

src/main.rs:
fn main() {}

";

    let result = format_project_summary(tree, file_contents);
    println!("Expected:\n{}\n", expected_output);
    println!("Actual:\n{}", result);
    assert_eq!(result, expected_output);
}

#[test]
fn test_deeply_nested_directories() {
    let files = vec![ScannedFile {
        path: "src/utils/math/helpers.rs".to_string(),
        content: Some("fn helper() {}".to_string()),
        is_binary: false,
    }];

    let expected = vec![
        ("└── src".to_string(), "src".to_string()),
        ("    └── utils".to_string(), "src/utils".to_string()),
        ("        └── math".to_string(), "src/utils/math".to_string()),
        (
            "            └── helpers.rs".to_string(),
            "src/utils/math/helpers.rs".to_string(),
        ),
    ];

    let result = build_tree(&files);
    println!("Expected:\n{:#?}", expected);
    println!("Actual:\n{:#?}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_directory_with_no_files() {
    let files = vec![];

    let expected: Vec<(String, String)> = vec![];

    let result = build_tree(&files);
    assert_eq!(result, expected);
}

#[test]
fn test_all_binary_files() {
    let files = vec![
        ScannedFile {
            path: "bin/image.png".to_string(),
            content: None,
            is_binary: true,
        },
        ScannedFile {
            path: "bin/sound.mp3".to_string(),
            content: None,
            is_binary: true,
        },
    ];

    let expected_content = HashMap::from([
        (
            "bin/image.png".to_string(),
            "bin/image.png:\n[Binary file]".to_string(),
        ),
        (
            "bin/sound.mp3".to_string(),
            "bin/sound.mp3:\n[Binary file]".to_string(),
        ),
    ]);

    let result = format_file_contents(&files);
    assert_eq!(result, expected_content);
}

#[test]

fn test_large_files_marked_correctly() {
    let files = vec![ScannedFile {
        path: "large.txt".to_string(),
        content: Some("[File size > 1.0MB (max: 1.0MB)]".to_string()), // Simulate large file
        is_binary: false,
    }];

    let expected_content = HashMap::from([(
        "large.txt".to_string(),
        "large.txt:\n[File size > 1.0MB (max: 1.0MB)]".to_string(),
    )]);

    let result = format_file_contents(&files);
    assert_eq!(result, expected_content);
}

#[test]
fn test_single_root_dir() {
    let files = vec![ScannedFile {
        path: "src/main.rs".to_string(),
        content: None,
        is_binary: false,
    }];

    let result = build_tree(&files);
    let expected = vec![
        ("└── src".to_string(), "src".to_string()),
        ("    └── main.rs".to_string(), "src/main.rs".to_string()),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_root_dirs() {
    let files = vec![
        ScannedFile {
            path: "src/main.rs".to_string(),
            content: None,
            is_binary: false,
        },
        ScannedFile {
            path: "assets/logo.png".to_string(),
            content: None,
            is_binary: true,
        },
    ];

    let result = build_tree(&files);
    // 'assets' < 'src', so assets is first => "├── assets", src is last => "└── src"
    let expected = vec![
        ("├── assets".to_string(), "assets".to_string()),
        (
            "│   └── logo.png".to_string(),
            "assets/logo.png".to_string(),
        ),
        ("└── src".to_string(), "src".to_string()),
        ("    └── main.rs".to_string(), "src/main.rs".to_string()),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_nested_dirs_standard_ascii() {
    let files = vec![ScannedFile {
        path: "src/utils/math/helpers.rs".to_string(),
        content: None,
        is_binary: false,
    }];

    let result = build_tree(&files);
    let expected = vec![
        ("└── src".to_string(), "src".to_string()),
        ("    └── utils".to_string(), "src/utils".to_string()),
        ("        └── math".to_string(), "src/utils/math".to_string()),
        (
            "            └── helpers.rs".to_string(),
            "src/utils/math/helpers.rs".to_string(),
        ),
    ];

    assert_eq!(result, expected);
}
