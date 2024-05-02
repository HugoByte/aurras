mod tests {
    use super::super::*;
    use runtime::{logger::CoreLogger, Logger};

    #[tokio::test]
    async fn test_filter_events_true() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "ICXTransfer".to_string(),
            contract_filter: "".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        // "0x4c98054" is taken from mainnet transaction history for testing
        let block = icon_service.get_block_by_height("0x4c98054").await.unwrap();

        let tx_list: Vec<Value> = serde_json::from_value(
            block
                .get("result")
                .and_then(|val| val.get("confirmed_transaction_list"))
                .unwrap()
                .clone(),
        )
        .unwrap();

        assert!(icon_feed.filter(&tx_list[1]).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_events_false() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "ICXIssued".to_string(),
            contract_filter: "".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        // tx hash is taken from mainnet transaction history for testing
        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(!icon_feed.filter(&tx.get("result").unwrap()).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_score_true() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "".to_string(),
            contract_filter: "cx21e94c08c03daee80c25d8ee3ea22a20786ec231".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(icon_feed.filter(&tx.get("result").unwrap()).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_score_false() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "".to_string(),
            contract_filter: "cx21e94c08c03daee80c25d8ee3ea22a20786ec231".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0xac8a02804ad4cdf65d9048de7b813f8d864a7d45c084328830491674e9ac7508",
            )
            .await
            .unwrap();

        assert!(!icon_feed.filter(&tx.get("result").unwrap()).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_event_and_score_true() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "Transfer".to_string(),
            contract_filter: "cx88fd7df7ddff82f7cc735c871dc519838cb235bb".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(icon_feed.filter(&tx.get("result").unwrap()).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_event_and_score_false() {
        let config = ChainConfig {
            chain: "Icon".to_string(),
            node_url: "https://api.icon.community/api/v3".to_string(),
            event_filter: "ICXIssued".to_string(),
            contract_filter: "cx88fd7df7ddff82f7cc735c871dc519838cb235bb".to_string(),
        };
        let logger = CoreLogger::new(Some("./event-feed.log"));
        let icon_feed = IconFeed::new(config, logger).unwrap();

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(!icon_feed.filter(&tx.get("result").unwrap()).await.unwrap());
    }

    #[tokio::test]
    async fn test_get_icon_block_height() {
        let icon_service = icon_service::IconService::new(None);

        let height = get_icon_block_height(&icon_service).await.unwrap();
        assert!(height > 0);
    }

    #[tokio::test]
    async fn test_get_event_logs_by_tx_hash_test() {
        // creates a new icon-service for mainnet
        let icon_service = icon_service::IconService::new(None);
        let event_logs = get_event_logs_by_tx_hash(
            &icon_service,
            // tx hash taken from mainnet transaction history
            "0x7dc777fad31b26ba12b2f4d0363cc08cdf702f5b52d852d685fe5463e9a4b3ba",
        )
        .await
        .unwrap();

        assert_eq!(
            event_logs[0].score_address,
            "cx85954d0dae92b63bf5cba03a59ca4ffe687bad0a"
        );
        assert_eq!(
            event_logs[0].indexed_events[0],
            "ICXTransfer(Address,Address,int)"
        );
        assert_eq!(event_logs[0].data, Vec::<String>::new());

        assert_eq!(
            event_logs[1].score_address,
            "cx85954d0dae92b63bf5cba03a59ca4ffe687bad0a"
        );
        assert_eq!(
            event_logs[1].indexed_events[0],
            "BuyTokens(int,int,Address,Address,int,int)"
        );
    }
}
