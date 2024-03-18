use wiremock::MockServer;

async fn create_server(add: &str) -> MockServer {
    let listener = std::net::TcpListener::bind(add).unwrap();
    MockServer::builder().listener(listener).start().await
}

pub async fn post(address: &str) -> MockServer {
    let server = create_server(address).await;
    server
}
