use contextor::scanner::scan_project;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

struct TestDir {
    path: PathBuf,
}

impl Drop for TestDir {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).expect("Failed to remove test directory");
        }
    }
}

fn setup_test_dir(test_name: &str) -> TestDir {
    let test_dir = PathBuf::from(format!("test_dir_{}", test_name));

    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).expect("Failed to remove old test directory");
    }

    fs::create_dir_all(&test_dir).expect("Failed to create test directory");

    TestDir { path: test_dir }
}

#[test]
fn test_scan_project() {
    let test_dir = setup_test_dir("scan_project");
    let test_file = test_dir.path.join("test.txt");
    fs::write(&test_file, "Hello, world!").unwrap();

    let results = scan_project(test_dir.path.to_str().unwrap(), 5_000_000);

    assert_eq!(results.len(), 1);

    // Compute the expected relative path correctly
    let expected_relative_path = test_file.strip_prefix(&test_dir.path).unwrap();

    assert_eq!(results[0].path, expected_relative_path.to_string_lossy());

    assert_eq!(results[0].content, Some("Hello, world!".to_string()));
    assert!(!results[0].is_binary);
}

#[test]
fn test_binary_detection() {
    let test_dir = setup_test_dir("binary_detection");
    let binary_file = test_dir.path.join("binary.bin");

    let mut file = fs::File::create(&binary_file).unwrap();
    file.write_all(&[0, 159, 146, 150]).unwrap(); // Some binary bytes

    let results = scan_project(test_dir.path.to_str().unwrap(), 5_000_000);

    assert_eq!(results.len(), 1);

    // Compute the expected relative path correctly
    let expected_relative_path = binary_file.strip_prefix(&test_dir.path).unwrap();

    assert_eq!(results[0].path, expected_relative_path.to_string_lossy());
    assert!(results[0].is_binary);
    assert_eq!(results[0].content, None);
}
