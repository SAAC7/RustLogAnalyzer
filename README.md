# Overview
As a software engineer looking to expand my knowledge of systems programming and memory safety, I am developing tools to deepen my understanding of the Rust ecosystem. This project demonstrates how to handle input/output operations, parse structured text, and efficiently implement custom logic in Rust.
The software written for this project is **RustLogAnalyzer**, a command-line interface (CLI) utility designed to read log files, classify individual lines by severity level (`ERROR`, `WARN`, `INFO`, `DEBUG`), and generate a comprehensive report.
My main goal in developing this software is to gain hands-on experience with Rust’s unique features, specifically exploring ownership, borrowing, custom data structures (`structs` and `enums`), pattern matching (`match`), and error handling.
Here you can find a demo of the software in action and a complete walkthrough of the code:

[Software Demo Video](https://youtu.be/hstF8qrHDRk)

# Development Environment
To develop this software, the following tools, language, and libraries were utilized:
- **Programming Language:** Rust (Stable)
- **Package Manager & Build Tool:** Cargo
- **Environment / IDE:** Visual Studio Code with the `rust-analyzer` extension and devcontainers
- **Operating System:** Linux (Arch Linux)
- **Standard Library Modules & Imports:**
  - `std::env`: Used for handling command-line arguments (`env::args()`).
  - `std::fs`: Used for file system operations, specifically reading log files into memory (`fs::read_to_string()`).
  - `std::io`: Used for input/output error management and handling `io::Error`.
  - `std::process`: Used for handling application exit codes (`process::exit(1)`).
  - `std::fmt`: Used for implementing formatting traits (`fmt::Display`) on custom log severity levels.
- **Internal Project Modules & Data Structures:**
  - `mod parser`: Handles line-by-line parsing (`parse_line`), custom `LogEntry` structures, and `LogLevel` enums.
  - `mod report`: Calculates statistical data and formats output reports via the `Report` structure.
  - Collections: Extensively utilizes vectors (`Vec`) to store and process parsed log entries.

# Useful Websites
Here is a list of websites and official documentation resources that I found helpful during the development of this project:
- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Learn Rust](https://rust-lang.org/es/learn/)

# Future Work
While the utility is fully functional, I have identified several areas for future improvement and expansion:
- Item 1: Implement multi-threaded file processing using channels (`std::sync::mpsc`) to analyze massive log files concurrently.
- Item 2: Add support for exporting the statistical report directly into JSON and CSV formats.
- Item 3: Introduce custom regular expressions (`regex` crate) for advanced pattern matching and filtering by timestamps or specific error codes.