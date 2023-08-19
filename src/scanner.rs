use std::fs;
use std::path::Path;

pub fn collect_llm_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() {
                    files.push(file_path.to_string_lossy().to_string());
                }
            }
        }
    }

    files
}

