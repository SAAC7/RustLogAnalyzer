//! Reporting module: calculates metrics and displays them in the console.
use crate::parser::{LogEntry, LogLevel};

/// Statistical summary of the analysis of a log file.

pub struct Report {
    /// Total number of lines in the file (including empty or invalid lines).
    pub total_lines: usize,
    /// Number of ERROR-level events.
    pub errors: usize,
    /// Number of WARN-level events.
    pub warnings: usize,
    /// Number of INFO-level events.
    pub infos: usize,
    /// Number of DEBUG-level events.
    pub debugs: usize,
    /// Lines that did not have a recognizable level.
    pub unrecognized: usize,
}
impl Report {
    /// Generates a report based on the extracted events.
    ///
    /// # Parameters
    /// * `total_lines` - Total number of lines read from the file.
    /// * `entries` - Slice containing the recognized events.
    ///
    /// # Returns
    /// A `Report` with all calculated counters.
    pub fn from_entries(total_lines: usize, entries: &[LogEntry]) -> Report {
        let mut report = Report {
            total_lines,
            errors: 0,
            warnings: 0,
            infos: 0,
            debugs: 0,
            unrecognized: total_lines - entries.len(),
        };
        for entry in entries {
            match entry.level {
                LogLevel::Error => report.errors += 1,
                LogLevel::Warn => report.warnings += 1,
                LogLevel::Info => report.infos += 1,
                LogLevel::Debug => report.debugs += 1,
            }
        }
        report
    }

    /// Calculates what percentage of the total a given quantity represents.
    ///
    /// # Parameters
    /// * `count` - Partial quantity.
    ///
    /// # Returns
    /// A percentage between 0.0 and 100.0 (0.0 if there are no lines).
    fn percentage(&self, count: usize) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        (count as f64 / self.total_lines as f64) * 100.0
    }
    /// Prints the formatted statistical report to the terminal.
    ///
    /// # Parameters
    /// * `file_name` - Name of the analyzed file (for display purposes only).
    pub fn print(&self, file_name: &str) {
        println!("==============================================");
        println!("        LOG ANALYSIS REPORT");
        println!("==============================================");
        println!("File analyzed :{}", file_name);
        println!("Total lines   :{}", self.total_lines);
        println!("{:<12} {:>6} {:>9}", "Level", "Total", "% lines");
        println!(
            "{:<12} {:>6} {:>8.1}%",
            "ERROR",
            self.errors,
            self.percentage(self.errors)
        );
        println!(
            "{:<12} {:>6} {:>8.1}%",
            "WARN",
            self.warnings,
            self.percentage(self.warnings)
        );
        println!(
            "{:<12} {:>6} {:>8.1}%",
            "INFO",
            self.infos,
            self.percentage(self.infos)
        );
        println!(
            "{:<12} {:>6} {:>8.1}%",
            "DEBUG",
            self.debugs,
            self.percentage(self.debugs)
        );
        println!(
            "{:<12} {:>6} {:>8.1}%",
            "No Level",
            self.unrecognized,
            self.percentage(self.unrecognized)
        );
        println!("==============================================");
    }
}

/// Prints details of critical events (ERROR and WARN).
///
/// # Parameters
/// * `entries` - A slice containing all recognized events.
pub fn print_critical(entries: &[LogEntry]) {
    println!("\n Critical events found:");

    // `filter` keeps only the elements that satisfy the condition.
    // `matches!` is a macro that asks: Does this value match this pattern?
    let critical: Vec<&LogEntry> = entries
        .iter()
        .filter(|e| matches!(e.level, LogLevel::Error | LogLevel::Warn))
        .collect();
    if critical.is_empty() {
        println!(" (any, ¡all right!)");
        return;
    }
    for entry in critical {
        println!(
            "  [línea {:>3}] {:<5} {}",
            entry.line_number, entry.level, entry.message
        );
    }
}
