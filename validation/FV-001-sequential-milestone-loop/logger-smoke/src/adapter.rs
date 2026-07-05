//! Stdout logging adapter — framework validation FV-001 M2.

use crate::port::LoggerPort;
use std::io::{self, Write};

/// Logger that writes messages to standard output.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StdoutLogger;

impl LoggerPort for StdoutLogger {
    fn log(&self, message: &str) {
        let mut stdout = io::stdout();
        let _ = writeln!(stdout, "{message}");
        let _ = stdout.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::LoggerPort;

    #[test]
    fn stdout_logger_implements_logger_port() {
        let logger = StdoutLogger;
        logger.log("framework-validation-m3");
    }

    #[test]
    fn stdout_logger_unit_struct_equality() {
        let a = StdoutLogger;
        let b = StdoutLogger;
        assert_eq!(a, b);
    }
}
