
use crate::storage::*;

#[derive(Debug)]
pub enum Consumer{
    pub pub_id : String,
    pub wasm : Vec<u8>,
    pub event_id : String,
    pub input_data : serde_json::Value,
}

impl Consumer{
    pub fn new() -> Self{
        Self{
            pub_id,
            wasm,
            event_id,
            input_data,
        }
    }
}

fn fetch_wasm(&mut self, event_id: Uuid) -> Result<(), CustomError> {
    let storage = CoreStorage::new("my-db")?; 
    self.wasm = storage.get_wasm(&event_id)?; 
    Ok(())
}


