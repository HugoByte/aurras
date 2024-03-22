use std::sync::{Mutex, MutexGuard, RwLockReadGuard};

use async_std::sync::RwLock;

use crate::{state_manager::CoreLogger, storage::CoreStorage};

pub trait Ctx: Send + 'static {
    fn get_logger(&self) -> std::sync::MutexGuard<CoreLogger>;
    fn get_db(&self) -> std::sync::MutexGuard<CoreStorage>;
}

pub struct Context {
    pub logger: Mutex<CoreLogger>,
    pub db : Mutex<CoreStorage>
}

impl Ctx for Context {
    fn get_logger(&self) -> std::sync::MutexGuard<CoreLogger> {
        self.logger.lock().unwrap() 
    }
    fn get_db(&self) -> std::sync::MutexGuard<CoreStorage> {
        self.db.lock().unwrap() 
    }
}

impl Context {
    pub fn new(logger: CoreLogger, db: CoreStorage) -> Self {
        Context {
            logger: Mutex::new(logger),
            db: Mutex::new(db),
        }
    }
}
