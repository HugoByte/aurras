use std::sync::{Mutex, MutexGuard};

use crate::{logger::CoreLogger, state_manager::{GlobalState, WorkflowState}, storage::CoreStorage};

type GlobalStateManager = GlobalState<WorkflowState, CoreLogger>; 

pub trait Ctx: Send + 'static {
    fn get_logger(&self) -> MutexGuard<CoreLogger>;
    fn get_db(&self) -> MutexGuard<CoreStorage>;
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager>;
}

pub struct Context {
    pub logger: Mutex<CoreLogger>,
    pub db : Mutex<CoreStorage>,
    pub state_manager : Mutex<GlobalStateManager>,
}

impl Ctx for Context {
    fn get_logger(&self) -> MutexGuard<CoreLogger> {
        self.logger.lock().unwrap() 
    }
    fn get_db(&self) -> MutexGuard<CoreStorage> {
        self.db.lock().unwrap() 
    }
    fn get_state_manager(&self) -> MutexGuard<GlobalStateManager>{
        self.state_manager.lock().unwrap()
    }

}

impl Context {
    pub fn new(logger: CoreLogger, db: CoreStorage, state_manager: GlobalStateManager) -> Self {
        Context {
            logger: Mutex::new(logger),
            db: Mutex::new(db),
            state_manager: Mutex::new(state_manager),
        }
    }
}
