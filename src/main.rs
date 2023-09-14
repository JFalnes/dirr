use chrono::{Duration, TimeZone, Utc};
use regex::Regex;
use std::{
    fs::{self, Metadata},
    path::{Path, PathBuf},
    time::SystemTime,
};

fn get_metadata(path: &PathBuf) -> Option<Metadata> {
    fs::metadata(path).ok()
}

fn format_file_size(size: u64) -> String {
    const BYTE: u64 = 1;
    const KILOBYTE: u64 = 1024 * BYTE;
    const MEGABYTE: u64 = 1024 * KILOBYTE;
    const GIGABYTE: u64 = 1024 * MEGABYTE;

    if size < KILOBYTE {
        format!("{} B", size)
    } else if size < MEGABYTE {
        format!("{:.2} KB", size as f64 / KILOBYTE as f64)
    } else if size < GIGABYTE {
        format!("{:.2} MB", size as f64 / MEGABYTE as f64)
    } else {
        format!("{:.2} GB", size as f64 / GIGABYTE as f64)
    }
}

fn format_metadata(metadata: &Metadata) -> String {
    if let Ok(modified_time) = metadata.modified() {
        let duration_since_epoch = match modified_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d,
            Err(_) => return String::from(" (Unable to fetch time before UNIX_EPOCH)"),
        };
        let file_size = metadata.len();

        let size_str = format_file_size(file_size);
        let time_str = format_time(duration_since_epoch);

        format!(" ({} modified {})", size_str, time_str)
    } else {
        String::from(" (Unable to fetch metadata)")
    }
}

fn format_time(duration_since_epoch: std::time::Duration) -> String {
    let now = Utc::now();
    let timestamp_result = Utc.timestamp_opt(duration_since_epoch.as_secs() as i64, 0);

    let file_time = match timestamp_result {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(dt1, _) => dt1,
        chrono::LocalResult::None => {
            return "an invalid time".to_string();
        }
    };

    let elapsed = now - file_time;

    if elapsed < Duration::minutes(1) {
        "just now".to_string()
    } else if elapsed < Duration::hours(1) {
        format!("{} minutes ago", elapsed.num_minutes())
    } else if elapsed < Duration::days(1) {
        format!("{} hours ago", elapsed.num_hours())
    } else {
        format!("{} days ago", elapsed.num_days())
    }
}

fn print_tree(paths: &[PathBuf], root: &Path, show_meta: bool) {
    for path in paths {
        if let Ok(display_path) = path.strip_prefix(root) {
            let depth = display_path.components().count();
            let prefix = "|   ".repeat(depth - 1);
            let meta_info = if show_meta {
                if let Some(metadata) = get_metadata(path) {
                    format_metadata(&metadata)
                } else {
                    String::from(" (Error fetching metadata)")
                }
            } else {
                String::new()
            };
            println!("{}|--{}{}", prefix, display_path.display(), meta_info);
        }
    }
}

fn generate_tree<P: AsRef<Path>>(
    path: P,
    exclude: &[Regex],
) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry_result in entries {
            let entry = entry_result?;
            let current_path = entry.path();
            if !is_excluded(&current_path, exclude) {
                results.push(current_path.clone());
                if current_path.is_dir() {
                    results.extend(generate_tree(&current_path, exclude)?);
                }
            }
        }
    }

    Ok(results)
}

fn is_excluded<P: AsRef<Path>>(path: P, exclude_patterns: &[Regex]) -> bool {
    path.as_ref()
        .to_string_lossy()
        .split(std::path::MAIN_SEPARATOR)
        .any(|comp| exclude_patterns.iter().any(|re| re.is_match(comp)))
}

fn print_help() {
    println!("dirr - A simple directory listing tool with exclusions and metadata support");
    println!();
    println!("Usage:");
    println!("  dirr [FLAGS] [PATTERNS]");
    println!();
    println!("Flags:");
    println!("  --help, -h      Shows this help message.");
    println!("  --meta, -m      Shows metadata (file size and modified time) alongside the directory listing.");
    println!("  --exclude, -x   Excludes directories that match the provided patterns. Supports regex patterns.");
    println!();
    println!("Examples:");
    println!("  dirr -m -x *tmp*");
    println!("    This will list all directories excluding those that have 'tmp' in their name and will show file metadata.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut show_meta = false;
    let mut exclude_patterns: Vec<Regex> = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--meta" | "-m" => {
                show_meta = true;
            }
            "--exclude" | "-x" => {
                let patterns = args
                    .split(|s| s == "--exclude" || s == "-x")
                    .last()
                    .unwrap_or(&[]);
                for pattern in patterns {
                    if let Ok(re) = Regex::new(pattern) {
                        exclude_patterns.push(re);
                    } else {
                        println!("Error: Invalid exclusion pattern '{}'.", pattern);
                        return;
                    }
                }
            }
            _ => {}
        }
    }

    let current_dir = PathBuf::from(".");
    match generate_tree(&current_dir, &exclude_patterns) {
        Ok(paths) => print_tree(&paths, &current_dir, show_meta),
        Err(e) => println!("Error: {}", e),
    }
}
