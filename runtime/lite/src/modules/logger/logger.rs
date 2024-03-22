use super::*;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

#[derive(Clone, Debug)]
pub struct CoreLogger {
    logger: slog::Logger,
}

impl CoreLogger {
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
                .append(true)
                .create(true)
                .open(file)
                .unwrap(),
            None => OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
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

impl Logger for CoreLogger {
    fn info(&self, msg: &str) {
        slog::info!(self.logger, "{msg:?}");
    }

    fn warn(&self, msg: &str) {
        slog::warn!(self.logger, "{msg:?}");
    }

    fn error(&self, msg: &str) {
        slog::error!(self.logger, "{msg:?}");
    }

    fn debug(&self, msg: &str) {
        slog::debug!(self.logger, "{msg:?}");
    }

    fn trace(&self, msg: &str) {
        slog::trace!(self.logger, "{msg:?}");
    }

    fn critical(&self, msg: &str) {
        slog::crit!(self.logger, "{msg:?}");
    }
}
