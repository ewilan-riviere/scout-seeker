use clap::{App, Arg};
use serde::Serialize;
use std::fs;
use walkdir::WalkDir;

pub mod test;

#[derive(Serialize)]
struct FileList {
    path: String,
    date: String,
    time_seconds: String,
    total_files: usize,
    files: Vec<String>,
}

fn main() {
    let matches = App::new("Scout")
        .version("0.1.10")
        .author("Ewilan Rivière")
        .about("Scout is a simple Rust CLI to scan a directory to list files, recursively.")
        .arg(
            Arg::with_name("directory")
                .help("Sets the directory to list files from")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file path as JSON, default is './output.json'")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose mode")
                .takes_value(false),
        )
        .arg(
            Arg::new("print")
                .short('p')
                .long("print")
                .help("Prints the list of files to the console, this option will clear all other console outputs")
                .takes_value(false),
        )
        .get_matches();

    let directory_path = matches.value_of("directory").unwrap();
    let current_dir = std::env::current_dir().unwrap();
    let default_json_path = format!("{}/output.json", current_dir.display());

    let output_file_path = matches.value_of("output").unwrap_or(&default_json_path);
    if !output_file_path.ends_with(".json") {
        println!("Output file must be a JSON file.");
        return;
    }

    let verbose = matches.is_present("verbose");
    let print = matches.is_present("print");

    if !print {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("scout v{}", VERSION);
    }

    if verbose && !print {
        println!("");
        println!("Scanning {} directory...", directory_path);
    }

    // check if the directory exists
    if !std::path::Path::new(directory_path).exists() {
        println!("Directory does not exist.");
        return;
    }

    // check if permission is granted
    if !std::path::Path::new(directory_path).is_dir() {
        println!("Permission denied.");
        return;
    }

    let start = std::time::Instant::now();
    let date = chrono::Local::now().to_string();
    let files = list_files_recursive(directory_path);

    if verbose && !print {
        println!("Scan completed!");

        println!("");
        println!("Directory: {}", directory_path);
        println!("Date: {:?}", date);
        println!("Time in seconds: {:?}", start.elapsed());
        println!("Total files: {}", files.len());
    }

    if !print {
        println!("");
        println!("Output file: {}", output_file_path);
    }

    if print {
        for file in &files {
            println!("{}", file);
        }
    }

    if !print {
        let file_list = FileList {
            path: directory_path.to_string(),
            date: date,
            time_seconds: start.elapsed().as_secs().to_string(),
            total_files: files.len(),
            files,
        };

        to_json(&file_list, output_file_path);
    }
}

fn to_json(file_list: &FileList, output_file: &str) -> () {
    // Convert the struct to a JSON string
    let json_string = serde_json::to_string(&file_list).expect("Failed to serialize to JSON");

    // Write JSON string to a file
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

                // Exclude files with names ending with a tilde
                if file_name.ends_with('~') {
                    continue;
                }

                // Exclude files with directories contains a dot at beginning, like .git
                if entry.path().display().to_string().contains("/.") {
                    continue;
                }

                files.push(entry.path().display().to_string());
            }
        }
    }

    files
}
