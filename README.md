# RustLogAnalyzer

## Description
A Rust CLI utility that reads a log file, classifies the lines
by severity (ERROR, WARN, INFO, DEBUG), and displays a statistical report.

## Requirements
- Rust (stable) and Cargo

## Compilation
    cargo build

## Execution
    cargo run -- example_server.logs

## Project Structure
- `src/main.rs`: main flow and argument handling
- `src/parser.rs`: types and line parsing
- `src/report.rs`: report calculation and output

## Demo Video
[Link to video](https://youtu.be/hstF8qrHDRk)

## Author
Saúl Abraham Arana Calderón
