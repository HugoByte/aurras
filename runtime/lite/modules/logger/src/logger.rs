use super::*;
use crate::traits::Logger;
use log::*;
use std::fs::File;

pub struct CoreLogger;

impl CoreLogger {
    pub fn new() -> CoreLogger {
        let target = Box::new(File::create("./log.log").expect("Can't create file"));

        env_logger::Builder::new()
            .target(env_logger::Target::Pipe(target))
            .filter(None, LevelFilter::Info)
            .filter(None, LevelFilter::Debug)
            .init();

        CoreLogger
    }
}

impl Logger for CoreLogger {
    fn info(&self, msg: &str) {
        log::info!("{}", msg);
    }

    fn warn(&self, msg: &str) {
        log::warn!("{}", msg);
    }

    fn error(&self, msg: &str) {
        log::error!("{}", msg);
    }

    fn debug(&self, msg: &str) {
        log::debug!("{}", msg);
    }
}
