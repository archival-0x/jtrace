//! logger.rs
//!
//!     JtraceLogger logging facility implementation
//!     for CLI verbosity.

use log::{Record, Level, Metadata};


/// empty struct representing logger, which will
/// implement the log::Log trait
pub struct JtraceLogger;


impl log::Log for JtraceLogger {

    // we always logging!
    fn enabled(&self, _metadata: &Metadata) -> bool { true }

    fn log(&self, record: &Record) {

        // determine prefix from log level
        let prefix = match record.level() {
            Level::Error    => "[ERROR] ",
            Level::Info     => "[INFO] ",
            _               => "[DEBUG] ",
        };

        // will always output
        if self.enabled(record.metadata()) {
            println!("{}{}", prefix, record.args());
        }
    }

    fn flush(&self) {}
}
