use serde::Serialize;
use std::env;
use std::fs;
use walkdir::WalkDir;

#[derive(Serialize)]
struct FileList {
    path: String,
    time_seconds: String,
    total_files: usize,
    files: Vec<String>,
}

fn main() {
    let start = std::time::Instant::now();

    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1];
    // let directory_path = "/Volumes/library/video/movies_animation";
    // let directory_path = "/Volumes/library/video/tv_shows";

    let files = list_files_recursive(directory_path);

    println!("Directory: {}", directory_path);
    println!("Time in seconds: {:?}", start.elapsed());
    println!("Total files: {}", files.len());

    let file_list = FileList {
        path: directory_path.to_string(),
        time_seconds: start.elapsed().as_secs().to_string(),
        total_files: files.len(),
        files,
    };

    to_json(&file_list);
}

fn to_json(file_list: &FileList) -> () {
    // Convert the struct to a JSON string
    let json_string = serde_json::to_string(&file_list).expect("Failed to serialize to JSON");

    // Write JSON string to a file
    let output_file = "./output.json";
    fs::remove_file(output_file).unwrap_or_default();
    fs::write(output_file, json_string).expect("Failed to write to file");
}

fn list_files_recursive(directory_path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    for entry in WalkDir::new(directory_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                // Exclude files with names beginning with a dot
                if file_name.starts_with('.') {
                    continue;
                }

                files.push(entry.path().display().to_string());
            }
        }
    }

    files
}
