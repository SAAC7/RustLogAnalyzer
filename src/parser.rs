//! Analysis module: defines the log data types
//! and the logic for converting a text line into a typed event.

use std::fmt;

/// Severity level of a log event.
///
/// An `enum` is a type that can be ONE of several possible variants.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}
impl LogLevel {
    /// Converts a word (token) to a log level, if it matches.
    ///
    /// # Parameters
    /// * `token` - Word to evaluate, for example `“ERROR”`.
    ///
    /// # Returns
    /// `Some(LogLevel)` if the word is a known level, `None` otherwise.
    pub fn from_token(token: &str) -> Option<LogLevel> {
        match token {
            "ERROR" => Some(LogLevel::Error),
            "WARN" => Some(LogLevel::Warn),
            "INFO" => Some(LogLevel::Info),
            "DEBUG" => Some(LogLevel::Debug),
            _ => None,
        }
    }
}

/// Allows you to print a `LogLevel` using `println!(“{}”, level)`.
impl fmt::Display for LogLevel {
    /// Writes the level name to the received formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        };
        write!(f, "{}", text)
    }
}
/// A single event extracted from the log file.
#[derive(Debug)]
pub struct LogEntry {
    /// Line number (starting at 1) where the event occurred.
    pub line_number: usize,
    /// Severity of the event.
    pub level: LogLevel,
    /// Descriptive text that follows the level.
    pub message: String,
}

/// Attempts to parse a line of text as a log event.
///
/// Searches for the first word that is a valid level; everything
/// that follows is considered the message.
///
/// # Parameters
/// * `line_number` - Original line number (for reporting).
/// * `line` - Line content.
///
/// # Returns
/// `Some(LogEntry)` if the line contains a recognized level,
/// `None` if it is empty or has an invalid format.
pub fn parse_line(line_number: usize, line: &str) -> Option<LogEntry> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    // `position` returns the index of the first token that meets the condition.
    // The `?` returns `None` immediately if none is found.
    let pos = tokens
        .iter()
        .position(|t| LogLevel::from_token(t).is_some())?;
    let level = LogLevel::from_token(tokens[pos])?;
    let message = tokens[pos + 1..].join(" ");

    Some(LogEntry {
        line_number,
        level,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detect_error() {
        let entry = parse_line(1, "2024-05-12 10:00:00 ERROR Disco lleno").unwrap();
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Disco lleno");
    }


    #[test]
    fn ignore_line_out_level(){
        assert!(parse_line(1, "corrupted line").is_none());
        assert!(parse_line(2, "").is_none());
    }
}
