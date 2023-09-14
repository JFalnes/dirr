use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

fn generate_tree<P: AsRef<Path>>(path: P, exclude: &[Regex]) -> Vec<PathBuf> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !is_excluded(&path, exclude) {
                    results.push(path.clone());
                    if path.is_dir() {
                        results.extend(generate_tree(&path, exclude));
                    }
                }
            }
        }
    }

    results
}

fn is_excluded<P: AsRef<Path>>(path: P, exclude_patterns: &[Regex]) -> bool {
    path.as_ref().components().any(|comp| {
        let comp_str = comp.as_os_str().to_string_lossy();
        exclude_patterns.iter().any(|re| re.is_match(&comp_str))
    })
}


fn print_tree(paths: &[PathBuf], root: &Path) {
    for path in paths {
        let display_path = path.strip_prefix(root).unwrap_or(path);
        let depth = display_path.components().count();
        let prefix = "|   ".repeat(depth - 1);
        println!("{}|--{}", prefix, display_path.display());
    }
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    println!("Reading directory: {}", current_dir.display());

    let mut args = std::env::args().peekable();
    let mut exclude = Vec::new();

    while let Some(arg) = args.next() {
        if arg == "-x" {
            while let Some(dir_to_exclude) = args.peek() {
                if dir_to_exclude.starts_with('-') {
                    break;
                }
                exclude.push(dir_to_exclude.clone());
                args.next(); // Consume the next argument since it's added to the exclude list
            }
            
            if exclude.is_empty() {
                eprintln!("Error: Expected at least one directory to exclude after `-x` flag.");
                return;
            }
        }
    }

    println!("Excluding directories: {:?}", exclude);

    // Compile the exclusion patterns into Regex
    let exclude_patterns: Vec<Regex> = exclude.iter().map(|pattern| {
        let regex_pattern = pattern.replace("*", ".*");
        Regex::new(&regex_pattern).expect("Failed to compile regex pattern")
    }).collect();

    // Modify the generate_tree function call to use the compiled patterns
    let paths = generate_tree(&current_dir, &exclude_patterns);
    print_tree(&paths, &current_dir);
}
