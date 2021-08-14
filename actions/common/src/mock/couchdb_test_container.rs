use harbourmaster::{Container, Error, Protocol};
use std::net::TcpListener;

pub struct CouchDB {
    container: Container,
    host_port: u16,
}

impl CouchDB {
    pub async fn new(user: String, pass: String) -> Result<Self, Error> {
        let host_port = get_unused_port()?;
        let container = Container::builder("couchdb")
            .pull_on_build(true)
            .name("couchdb")
            .slug_length(6)
            .environment_variable(format!("COUCHDB_USER={}", user))
            .environment_variable(format!("COUCHDB_PASSWORD={}", pass))
            .expose(5984, host_port, Protocol::Tcp)
            .build()
            .await?;

        Ok(Self {
            container,
            host_port,
        })
    }

    pub fn port(&self) -> u16 {
        self.host_port
    }

    pub async fn delete(self) -> Result<(), Error> {
        self.container.delete().await?;
        Ok(())
    }
}

fn get_unused_port() -> Result<u16, std::io::Error> {
    let listener = TcpListener::bind("localhost:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}
