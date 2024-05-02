use std::sync::{Mutex, MutexGuard};

use crate::{
    logger::CoreLogger,
    state_manager::{GlobalState, WorkflowState},
    storage::CoreStorage,
};

type GlobalStateManager = GlobalState<WorkflowState, CoreLogger>;

/// Represents the runtime context.
pub trait Ctx: Send + 'static {
    /// Returns a reference of the logger.
    fn get_logger(&self) -> MutexGuard<CoreLogger>;

    /// Returns a reference to the database.
    fn get_db(&self) -> MutexGuard<CoreStorage>;

    /// Returns a reference to the cache database.
    fn get_cache_db(&self) -> MutexGuard<CoreStorage>;

    /// Returns a reference to the state manager.
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager>;
}

/// Represents the context of the runtime. This is the main entry point for the runtime.
pub struct Context {
    pub logger: Mutex<CoreLogger>,
    pub db: Mutex<CoreStorage>,
    pub cache_db: Mutex<CoreStorage>,
    pub state_manager: Mutex<GlobalStateManager>,
}

impl Ctx for Context {
    /// Returns a reference of the logger.
    fn get_logger(&self) -> MutexGuard<CoreLogger> {
        self.logger.lock().unwrap()
    }

    /// Returns a reference to the database.
    fn get_db(&self) -> MutexGuard<CoreStorage> {
        self.db.lock().unwrap()
    }

    /// Returns a reference to the cache database.
    fn get_cache_db(&self) -> MutexGuard<CoreStorage> {
        self.cache_db.lock().unwrap()
    }

    /// Returns a reference to the state manager.
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager> {
        self.state_manager.lock().unwrap()
    }
}

impl Context {
    /// Creates a new instance of the Context struct.
    ///
    /// # Arguments
    ///
    /// * `logger`: Logger of the runtime.
    /// * `db`: Database of the runtime.
    /// * `state_manager`: State manager of the runtime.
    pub fn new(
        logger: CoreLogger,
        db: CoreStorage,
        cache_db: CoreStorage,
        state_manager: GlobalStateManager,
    ) -> Self {
        Context {
            logger: Mutex::new(logger),
            db: Mutex::new(db),
            cache_db: Mutex::new(cache_db),
            state_manager: Mutex::new(state_manager),
        }
    }
}
