//! Logging port — framework validation FV-001 M1.

/// Incidental exercise artifact — not the validation target.
pub trait LoggerPort {
    /// Emit a log message.
    fn log(&self, message: &str);
}
