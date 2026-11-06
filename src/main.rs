//! RustLogAnalyzer: CLI utility to analize log files.

mod parser;
mod report;

use std::env;
use std::fs;
use std::io;
use std::process;

use parser::LogEntry;
use report::Report;

/// Retrieves the file path from the command-line arguments.
///
/// # Parameters
/// * `args` - All program arguments (index 0 is the executable).
///
/// # Returns
/// `Some(&str)` with the path if it was provided, `None` otherwise.

fn get_path_from_args(args: &[String]) -> Option<&str> {
    args.get(1).map(|s| s.as_str())
}

/// Reads the entire contents of a text file.
///
/// # Parameters
/// * `path` - Path to the file to be read.
///
/// # Returns
/// `Ok(String)` with the content, or `Err(io::Error)` if the file
/// does not exist, lacks permissions, or is not valid UTF-8 text.

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Program entry point.
fn main() {
    // 1. Receiving arguments.
    let args: Vec<String> = env::args().collect();

    let path = match get_path_from_args(&args) {
        Some(p) => p,
        None => {
            eprintln!("Use: cargo run -- <file_path>");
            process::exit(1);
        }
    };

    // 2. Safely read the file.
    let content = match read_file(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("Error reading '{}': {}", path, error);
            process::exit(1);
        }
    };

    // 3. Line-by-line analysis.
    let total_lines = content.lines().count();
    let entries: Vec<LogEntry> = content
        .lines()
        .enumerate()
        .filter_map(|(index, line)| parser::parse_line(index + 1, line))
        .collect();

    // 4. Generating the report.
    let report = Report::from_entries(total_lines, &entries);
    report.print(path);
    report::print_critical(&entries);
}
