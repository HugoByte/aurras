mod tests {
    use super::super::*;
    use crate::common::ChainConfig;
    use runtime::{logger::CoreLogger};

    #[tokio::test]
    async fn test_cosmos_feed_new() {
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "transfer".to_string(),
            contract_filter: "".to_string(),
            chain: "cosmos".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));

        let cosmos_feed = CosmosFeed::new(chain_config.clone(), logger);
        assert_eq!(cosmos_feed.chain_config, chain_config);
        assert_eq!(cosmos_feed.events.len(), 1);
        assert!(cosmos_feed.events.contains(&"transfer".to_string()));
    }

    #[tokio::test]
    async fn test_event_feed() {
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "event1,event2".to_string(),
            contract_filter: "contract1,contract2".to_string(),
            chain: "cosmos".to_string(),
        };

        let callback = |events: Vec<serde_json::Value>| {
            assert!(events.is_empty());
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let cosmos_feed = CosmosFeed::new(chain_config.clone(), logger);
        let result = cosmos_feed.event_feed(&callback).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_event_feed_error() {
        // Mock chain config and WebSocketClient
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "event1,event2".to_string(),
            contract_filter: "contract1,contract2".to_string(),
            chain: "cosmos".to_string(),
        };

        let callback = |events: Vec<serde_json::Value>| {
            // Simulate an error condition
            assert!(events.is_empty());
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let cosmos_feed = CosmosFeed::new(chain_config.clone(), logger);

        let _ = cosmos_feed.event_feed(&callback).await;
    }
}
