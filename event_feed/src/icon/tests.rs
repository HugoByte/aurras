mod tests {
    use super::super::*;

    #[tokio::test]
    async fn test_filter_events_true() {
        let filter = Filter {
            events: Some(vec!["ICXTransfer".to_string()]),
            score: None,
        };

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

        assert!(filter.filter(&icon_service, &tx_list[1]).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_events_false() {
        let filter = Filter {
            events: Some(vec!["ICXIssued".to_string()]),
            score: None,
        };

        let icon_service = IconService::new(None);

        // tx hash is taken from mainnet transaction history for testing
        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(!filter
            .filter(&icon_service, &tx.get("result").unwrap())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_filter_score_true() {
        let filter = Filter {
            events: None,
            score: Some(vec![
                "cx21e94c08c03daee80c25d8ee3ea22a20786ec231".to_string()
            ]),
        };

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(filter
            .filter(&icon_service, &tx.get("result").unwrap())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_filter_score_false() {
        let filter = Filter {
            events: None,
            score: Some(vec![
                "cx21e94c08c03daee80c25d8ee3ea22a20786ec231".to_string()
            ]),
        };

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0xac8a02804ad4cdf65d9048de7b813f8d864a7d45c084328830491674e9ac7508",
            )
            .await
            .unwrap();

        assert!(!filter
            .filter(&icon_service, &tx.get("result").unwrap())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_filter_event_and_score_true() {
        let filter = Filter {
            events: Some(vec!["Transfer".to_string()]),
            score: Some(vec![
                "cx88fd7df7ddff82f7cc735c871dc519838cb235bb".to_string()
            ]),
        };

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(filter
            .filter(&icon_service, &tx.get("result").unwrap())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_filter_event_and_score_false() {
        let filter = Filter {
            events: Some(vec!["ICXIssued".to_string()]),
            score: Some(vec![
                "cx88fd7df7ddff82f7cc735c871dc519838cb235bb".to_string()
            ]),
        };

        let icon_service = IconService::new(None);

        let tx = icon_service
            .get_transaction_by_hash(
                "0x10d2df93d78b1bea76edf0fa33114ba94e871e09b562b2235548de23185c0e6b",
            )
            .await
            .unwrap();

        assert!(!filter
            .filter(&icon_service, &tx.get("result").unwrap())
            .await
            .unwrap());
    }

    #[ignore = "ssb server must be running"]
    #[tokio::test]
    async fn test_create_ssb_producer() {
        dotenv::dotenv().ok();
        let ssb_ip = env::var("PRODUCER_IP").ok();
        let ssb_port = env::var("PRODUCER_PORT").ok();
        let ssb_secret = env::var("PRODUCER_SECRET").ok();

        let _ = Producer::create_ssb_producer(
            ssb_ip.as_deref(),
            ssb_port.as_deref(),
            ssb_secret.as_deref(),
        )
        .await
        .unwrap();
    }

    #[ignore = "ssb server must be running and producer invite must be provided"]
    #[tokio::test]
    async fn test_accept_invite() {
        dotenv::dotenv().ok();
        let ssb_ip = env::var("PRODUCER_IP").ok();
        let ssb_port = env::var("PRODUCER_PORT").ok();
        let ssb_secret = env::var("PRODUCER_SECRET").ok();
        let invite = env::var("PRODUCER_INVITE").expect("producer invite must be provided");

        let mut producer = Producer::create_ssb_producer(
            ssb_ip.as_deref(),
            ssb_port.as_deref(),
            ssb_secret.as_deref(),
        )
        .await
        .unwrap();

        producer.accept_invite(&invite).await.unwrap();
    }

    #[ignore = "ssb server must be running and producer invite must be provided"]
    #[tokio::test]
    async fn test_publish_feed() {
        dotenv::dotenv().ok();
        let ssb_ip = env::var("PRODUCER_IP").ok();
        let ssb_port = env::var("PRODUCER_PORT").ok();
        let ssb_secret = env::var("PRODUCER_SECRET").ok();
        let invite = env::var("PRODUCER_INVITE").expect("producer invite must be provided");

        let mut producer = Producer::create_ssb_producer(
            ssb_ip.as_deref(),
            ssb_port.as_deref(),
            ssb_secret.as_deref(),
        )
        .await
        .unwrap();

        producer.accept_invite(&invite).await.unwrap();

        let pub_address = get_ssb_pub_address_from_invite(&invite).unwrap();

        producer
            .publish_feed(&pub_address, serde_json::json!("hello_world"))
            .await
            .unwrap();
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

    #[test]
    fn test_get_list_env_var() {
        env::set_var("EVENTS", "ICXTransfer,BuyTokens,ICXIssued");
        let events = get_list_env_var("EVENTS").unwrap();
        assert_eq!(
            events.as_ref(),
            vec!["ICXTransfer", "BuyTokens", "ICXIssued"]
        );
    }

    #[test]
    fn test_get_list_env_var_none_value() {
        env::remove_var("EVENTS");
        let events = get_list_env_var("EVENTS");
        assert!(events.is_none());
    }

    #[test]
    fn test_get_ssb_pub_address_from_invite() {
        let invite = "172.18.0.5:8008:@TTss8d+vQgw19tImFYFTPL9N3aYBUrJiyEh4ha9nGvY=.ed25519~gf2tRTWNzMApD3z/6mSQJUwFAYv68whYtMFgdy/zpBE=";
        let pub_address = get_ssb_pub_address_from_invite(invite).unwrap();

        assert_eq!(
            pub_address,
            "@TTss8d+vQgw19tImFYFTPL9N3aYBUrJiyEh4ha9nGvY=.ed25519"
        );
    }
}
