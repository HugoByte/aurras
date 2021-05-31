use chesterfield::sync::Database;
use serde_json::{Value, Error};
use std::env;

pub struct Context {
    pub host: String,
    db: Database,
}


#[cfg(test)]
impl Context {
    pub fn new(db: Database) -> Self {
        let host = if env::var("__OW_API_HOST").is_ok() {
            env::var("__OW_API_HOST").unwrap()
        } else {
            "host.docker.internal".to_string()
        };
        Context { host, db }
    }

    pub fn insert_document(&mut self, mut doc: Value) -> Result<String, String> {
        match self.db.insert(&mut doc, None).send() {
            Ok(r) => {
                return Ok(r.id)
            }
            Err(err) => return Err(format!("error creating document {}: {:?}", doc, err)),
        };
    }

    pub fn get_document(&self, id: &str) -> Result<Value, Error> {
        match self.db.get(id).send::<Value>() {
            Ok(v) => return Ok(v.into_inner().unwrap()),
            Err(err) => return Err(format!("error fetching document {}: {:?}", id, err)).map_err(serde::de::Error::custom),
        }
    }
}

#[cfg(not(test))]
impl Context {
    pub fn new(db: Database) -> Self {
        let host = if env::var("__OW_API_HOST").is_ok() {
            env::var("__OW_API_HOST").unwrap()
        } else {
            "host.docker.internal".to_string()
        };
        Context { host, db }
    }

    pub fn insert_document(&mut self, mut doc: Value) -> Result<String, String> {
        match self.db.insert(&mut doc, None).send() {
            Ok(r) => {
                return Ok(r.id)
            }
            Err(err) => return Err(format!("error creating document {}: {:?}", doc, err)),
        };
    }

    pub fn get_document(&self, id: &str) -> Result<Value, Error> {
        match self.db.get(id).send::<Value>() {
            Ok(v) => return Ok(v.into_inner().unwrap()),
            Err(err) => return Err(format!("error fetching document {}: {:?}", id, err)).map_err(serde::de::Error::custom),
        }
    }
}
