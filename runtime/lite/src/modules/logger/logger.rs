use super::*;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

/// Represents the core logger which is a wrapper around the `slog` logger.
#[derive(Clone, Debug)]
pub struct CoreLogger {
    logger: slog::Logger,
}

impl CoreLogger {
    /// Creates a new `CoreLogger` instance.
    ///
    /// # Arguments
    ///
    /// * `log_file`: A optional path to a log file. If not provided, the current path with the
    /// name `workflows.log` filename will be used.
    ///
    /// # Returns
    ///
    /// Returns a new `CoreLogger` instance.
    pub fn new(log_file: Option<&str>) -> CoreLogger {
        use std::fs::OpenOptions;

        let level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());
        let level = match level.to_lowercase().as_str() {
            "info" => slog::Level::Info,
            "debug" => slog::Level::Debug,
            "error" => slog::Level::Error,
            "warn" => slog::Level::Warning,
            "trace" => slog::Level::Trace,
            "critical" => slog::Level::Critical,
            _ => slog::Level::Info,
        };

        let file = match log_file {
            Some(file) => OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .unwrap(),
            None => OpenOptions::new()
                .create(true)
                .write(true)
                .open("./workflows.log")
                .unwrap(),
        };

        let decorator = slog_term::PlainDecorator::new(file);
        let file_drain = slog_term::FullFormat::new(decorator).build().fuse();

        let decorator = slog_term::TermDecorator::new().build();
        let terminal_drain = slog_term::FullFormat::new(decorator).build().fuse();

        let drain = slog::Duplicate::new(file_drain, terminal_drain).fuse();

        let drain = slog_async::Async::new(drain)
            .overflow_strategy(slog_async::OverflowStrategy::Block)
            .build()
            .filter_level(level)
            .fuse();
        let logger = slog::Logger::root(drain, slog::o!());

        CoreLogger { logger }
    }
}

/// Implements the `Logger` trait for the `CoreLogger`.
impl Logger for CoreLogger {
    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn info(&self, msg: &str) {
        slog::info!(self.logger, "{msg:?}");
    }

    /// Logs a message at the `warning` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn warn(&self, msg: &str) {
        slog::warn!(self.logger, "{msg:?}");
    }

    /// Logs a message at the `error` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn error(&self, msg: &str) {
        slog::error!(self.logger, "{msg:?}");
    }

    /// Logs a message at the `debug` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn debug(&self, msg: &str) {
        slog::debug!(self.logger, "{msg:?}");
    }

    /// Logs a message at the `trace` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn trace(&self, msg: &str) {
        slog::trace!(self.logger, "{msg:?}");
    }

    /// Logs a message at the `critical` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn critical(&self, msg: &str) {
        slog::crit!(self.logger, "{msg:?}");
    }
}
