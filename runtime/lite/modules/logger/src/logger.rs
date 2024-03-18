use super::*;
use crate::traits::Logger;
use env_logger::*;
use log::*;

pub struct CoreLogger {
    logger: env_logger::Logger,
}

impl CoreLogger {
    pub fn new() -> CoreLogger {
        Builder::from_default_env()
            .filter_module("logger::logger", LevelFilter::Info)
            .filter_module("logger::logger", LevelFilter::Debug)
            .target(Target::Stdout)
            .init();

        let logger = Builder::from_default_env().build();

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
