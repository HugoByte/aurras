use std::sync::{Mutex, MutexGuard};

use crate::{
    logger::CoreLogger,
    state_manager::{GlobalState, WorkflowState},
    storage::CoreStorage,
};

type GlobalStateManager = GlobalState<WorkflowState, CoreLogger>;

/// Represents the context of the runtime. This trait defines the methods that are used to access the context.
pub trait Ctx: Send + 'static {
    /// Returns a reference to the logger.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<CoreLogger>` that represents the reference to the logger for the runtime.
    fn get_logger(&self) -> MutexGuard<CoreLogger>;

    /// Returns a reference to the database.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<CoreStorage>` that represents the reference to the database for the runtime.
    fn get_db(&self) -> MutexGuard<CoreStorage>;

    /// Returns a reference to the state manager.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<GlobalStateManager>` that represents the reference to the state manager
    /// for the runtime.
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager>;
}

/// Represents the context of the runtime. This is the main entry point for the runtime.
///
/// # Properties
///
/// * `logger`: An instance of `MutexGuard<CoreLogger>` that represents the logger for the runtime.
/// * `db`: An instance of `MutexGuard<CoreStorage>` that represents the database for the runtime.
/// * `state_manager`: An instance of `MutexGuard<GlobalStateManager>` that represents the state manager for the runtime.
pub struct Context {
    pub logger: Mutex<CoreLogger>,
    pub db: Mutex<CoreStorage>,
    pub state_manager: Mutex<GlobalStateManager>,
}

/// Implements the `Ctx` trait for the `Context` struct.
impl Ctx for Context {
    /// Returns a reference to the logger.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<CoreLogger>` that represents the reference to the logger for the runtime.
    fn get_logger(&self) -> MutexGuard<CoreLogger> {
        self.logger.lock().unwrap()
    }

    /// Returns a reference to the database.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<CoreStorage>` that represents the reference to the database for the runtime.
    fn get_db(&self) -> MutexGuard<CoreStorage> {
        self.db.lock().unwrap()
    }

    /// Returns a reference to the state manager.
    ///
    /// # Returns
    ///
    /// Returns instance of `MutexGuard<GlobalStateManager>` that represents the reference to the state manager
    /// for the runtime.
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager> {
        self.state_manager.lock().unwrap()
    }
}

/// Implements the Context struct.
impl Context {
    /// Creates a new instance of the Context struct.
    ///
    /// # Arguments
    ///
    /// * `logger`: An instance of `CoreLogger` that represents the logger for the runtime.
    /// * `db`: An instance of `CoreStorage` that represents the database for the runtime.
    /// * `state_manager`: An instance of `GlobalStateManager` that represents the state manager for the runtime.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `Context` struct.
    pub fn new(logger: CoreLogger, db: CoreStorage, state_manager: GlobalStateManager) -> Self {
        Context {
            logger: Mutex::new(logger),
            db: Mutex::new(db),
            state_manager: Mutex::new(state_manager),
        }
    }
}
