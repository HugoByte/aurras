mod tests {
    use super::super::*;

    #[tokio::test]
    async fn test_cosmos_feed_new() {
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "transfer".to_string(),
            contract_filter: "".to_string(),
        };
        let cosmos_feed = CosmosFeed::new(chain_config.clone());

        assert_eq!(cosmos_feed.chain_config, chain_config);
        assert_eq!(cosmos_feed.events.len(), 1);
        assert!(cosmos_feed.events.contains(&"transfer".to_string()));
    }

    #[tokio::test]
    async fn test_event_feed_success() {
        // Mock chain config and WebSocketClient
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "".to_string(),
            contract_filter: "".to_string(),
        };

        let (ws_client, ws_driver) = WebSocketClient::new("ws://127.0.0.1:26657/websocket")
            .await
            .unwrap();

        ws_client
            .subscribe(EventType::NewBlock.into())
            .await
            .unwrap();

        let result = CosmosFeed::event_feed(chain_config.clone()).await;
        let block_height = ws_client.latest_block().await;

        // Assert the result
        assert!(block_height.is_ok());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_event_feed_error() {
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26654".to_string(),
            event_filter: "".to_string(),
            contract_filter: "".to_string(),
        };

        let result = CosmosFeed::event_feed(chain_config.clone()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_filter_event() {
        let event_name = vec!["rewards"];
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "event1,event2".to_string(),
            contract_filter: "".to_string(),
        };
        let (client, driver) = WebSocketClient::new("ws://127.0.0.1:26657/websocket")
            .await
            .unwrap();

        let result = CosmosFeed::filter_event(&event_name, client, chain_config).await;
        let mut iter = result.iter().filter_map(|s| Some(s));
        // Assert the result
        assert_eq!(iter.next(), Some(()).as_ref());
    }

    #[should_panic]
    #[tokio::test]
    async fn test_filter_event_with_no_matching_event() {
        let event_name = vec!["event3"];
        let chain_config = ChainConfig {
            node_url: "ws://127.0.0.1:26657/websocket".to_string(),
            event_filter: "event1,event2".to_string(),
            contract_filter: "".to_string(),
        };
        let (client, _driver) = WebSocketClient::new("ws://127.0.0.1:26657/websocket")
            .await
            .unwrap();

        let _ = CosmosFeed::filter_event(&event_name, client, chain_config).await;
    }
}
