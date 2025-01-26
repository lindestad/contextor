use crate::scanner::ScannedFile;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Default)]
struct Node {
    children: BTreeMap<String, Node>, // Subdirectories
    files: Vec<String>,               // Filenames
}

impl Node {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            files: vec![],
        }
    }

    // Insert a path like ["src", "utils", "math", "helpers.rs"]
    fn insert_path(&mut self, parts: &[&str]) {
        if parts.is_empty() {
            return;
        }

        if parts.len() == 1 {
            // It's a file
            self.files.push(parts[0].to_string());
        } else {
            // It's a directory
            let dir = parts[0];
            let node = self
                .children
                .entry(dir.to_string())
                .or_insert_with(Node::new);
            node.insert_path(&parts[1..]);
        }
    }
}

pub fn build_tree(files: &[ScannedFile]) -> Vec<(String, String)> {
    // 1) Build an in-memory tree of top-level dirs
    let mut root_map: BTreeMap<String, Node> = BTreeMap::new();
    let mut top_dirs: BTreeSet<String> = BTreeSet::new();

    // Track files in "." if there's no directory
    root_map.entry(".".to_string()).or_insert_with(Node::new);

    // Identify top-level dirs and build placeholders
    for file in files {
        let parts: Vec<&str> = file.path.split('/').collect();
        if parts.len() == 1 {
            // File in "."
            let node = root_map.get_mut(".").unwrap();
            node.insert_path(&parts);
        } else {
            top_dirs.insert(parts[0].to_string());
        }
    }

    // Ensure each top-level dir is in root_map
    for dir in top_dirs {
        root_map.entry(dir).or_insert_with(Node::new);
    }

    // Now insert each file path
    for file in files {
        let parts: Vec<&str> = file.path.split('/').collect();
        if parts.len() > 1 {
            let top = parts[0];
            if let Some(node) = root_map.get_mut(top) {
                node.insert_path(&parts[1..]);
            }
        }
    }

    // 2) Collect final output here
    let mut output = Vec::new();

    // 3) DFS function printing each node
    fn dfs_print(
        full_path: &str, // e.g. "src/utils"
        node: &Node,
        prefix: &str, // e.g. "│   "
        output: &mut Vec<(String, String)>,
        is_last: bool, // whether this is the last sibling
    ) {
        // --- Derive the label from full_path ---
        let label = match full_path.rsplit_once('/') {
            Some((_, last)) => last,
            None => full_path, // no slash found
        };

        // --- Print this node (unless it's just ".") ---
        if label != "." {
            let connector = if is_last { "└── " } else { "├── " };
            let line = format!("{}{}{}", prefix, connector, label);
            output.push((line, full_path.to_string()));
        }

        // --- Build new prefix for children ---
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        // Sort subdirectories & files
        let mut dirs: Vec<_> = node.children.keys().cloned().collect();
        dirs.sort();
        let mut files = node.files.clone();
        files.sort();

        // We'll figure out how many children we have total
        let total_children = dirs.len() + files.len();
        let mut index = 0;

        // --- Print subdirectories ---
        for d in dirs {
            index += 1;
            let child_is_last = index == total_children;
            let child_node = &node.children[&d];

            // Child’s path is "full_path/d" unless full_path == "." or ""
            let child_full_path = if full_path == "." || full_path.is_empty() {
                d.clone()
            } else {
                format!("{}/{}", full_path, d)
            };

            dfs_print(
                &child_full_path,
                child_node,
                &new_prefix,
                output,
                child_is_last,
            );
        }

        // --- Print files ---
        for f in files {
            index += 1;
            let file_is_last = index == total_children;
            let connector = if file_is_last {
                "└── "
            } else {
                "├── "
            };
            let line = format!("{}{}{}", new_prefix, connector, f);

            let file_full_path = if full_path == "." || full_path.is_empty() {
                f.clone()
            } else {
                format!("{}/{}", full_path, f)
            };
            output.push((line, file_full_path));
        }
    }

    // 4) Print the directories in sorted order
    let mut top_keys: Vec<_> = root_map.keys().cloned().collect();
    top_keys.sort();

    // Grab the real top-level directories (excluding ".")
    let top_dirs_no_dot: Vec<_> = top_keys
        .iter()
        .filter(|k| k.as_str() != ".")
        .cloned()
        .collect();

    // Print any files in "." first
    if let Some(dot_node) = root_map.get(".") {
        if !dot_node.files.is_empty() || !dot_node.children.is_empty() {
            // We treat "." as if it's a node, but skip printing its label
            dfs_print(".", dot_node, "", &mut output, false);
        }
    }

    // Then print each real top-level directory
    let count = top_dirs_no_dot.len();
    for (i, key) in top_dirs_no_dot.iter().enumerate() {
        let node = &root_map[key];
        let is_last = i == (count - 1);

        // For example, if `key == "src"`, pass that as the path
        dfs_print(key, node, "", &mut output, is_last);
    }

    output
}

pub fn format_file_contents(files: &[ScannedFile]) -> HashMap<String, String> {
    let mut formatted_output = HashMap::new();

    // Collect and sort file paths first
    let mut sorted_files: Vec<&ScannedFile> = files.iter().collect();
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

    for file in sorted_files {
        let formatted_entry = if file.is_binary {
            "[Binary file]".to_string()
        } else {
            file.content.clone().unwrap_or("[Empty file]".to_string())
        };

        formatted_output.insert(
            file.path.clone(),
            format!("{}:\n{}", file.path, formatted_entry),
        );
    }

    formatted_output
}

pub fn format_project_summary(
    tree: Vec<(String, String)>,
    file_contents: HashMap<String, String>,
) -> String {
    let mut output = String::new();

    // Append tree structure first
    for (formatted_entry, _) in &tree {
        output.push_str(formatted_entry);
        output.push('\n');
    }

    output.push('\n'); // Space between tree and file contents

    let mut sorted_files: Vec<_> = file_contents.keys().collect();
    sorted_files.sort();

    for file in sorted_files {
        if let Some(content) = file_contents.get(file) {
            output.push_str(content);
            output.push_str("\n\n");
        }
    }

    output
}
