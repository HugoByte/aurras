
use std::sync::Arc;

use super::*;

pub struct Context<T : EventFeeder + ?Sized>{
    pub feed_client: Arc<T>,
}

impl<T: EventFeeder+ ?Sized> Context<T> {
    pub fn new(client: Arc<T>) -> Self{
        Context { feed_client: client }
    }
}
