use super::*;
use crate::traits::Logger;
use env_logger::*;
use log::*;

pub struct CoreLogger {
    logger: env_logger::Logger,
}

impl CoreLogger {
    pub fn new(log_file: Option<&str>) -> CoreLogger {
        use std::fs::OpenOptions;

       let file =  match log_file {
            Some(file) => {
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(file)
                    .unwrap()
            }
            None => {
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open("./workflows.log")
                    .unwrap()
            }
        };

        let decorator = slog_term::PlainDecorator::new(file);
        let file_drain = slog_term::FullFormat::new(decorator).build().fuse();

        let decorator = slog_term::TermDecorator::new().build();
        let terminal_drain = slog_term::FullFormat::new(decorator).build().fuse();

        let drain = slog::Duplicate::new(file_drain, terminal_drain).fuse();

        let drain = slog_async::Async::new(drain)
            .overflow_strategy(slog_async::OverflowStrategy::Block)
            .build()
            .fuse();
        let logger = slog::Logger::root(drain, slog::o!());

        CoreLogger { logger }
    }
}

impl Logger for CoreLogger {
    fn info(&self, msg: &str) {
        log::info!("{msg:?}");
    }

    fn warn(&self, msg: &str) {
        log::warn!("{msg:?}");
    }

    fn error(&self, msg: &str) {
        log::error!("{msg:?}");
    }

    fn debug(&self, msg: &str) {
        log::debug!("{msg:?}");
    }
}