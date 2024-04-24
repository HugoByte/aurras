use tests::types::PolkadotFeed;

use super::*;
#[cfg(test)]
use crate::ChainConfig;
#[tokio::test]
async fn test_filter_events() {
    let config = ChainConfig {
        chain: "Susbtrate".to_string(),
        node_url: "wss://rococo-rpc.polkadot.io".to_string(),
        event_filter: "balances=Transfer".to_string(),
        contract_filter: "".to_string(),
    };
    let polkadot = PolkadotFeed::<PolkadotConfig>::new(config).await;
    let result = polkadot.split_filter();

    assert_eq!(
        result,
        std::collections::HashMap::from([("balances".to_string(), vec!["Transfer"])])
    );
}

#[tokio::test]
async fn test_filter_events_argument_with_no_method() {
    let config = ChainConfig {
        chain: "Susbtrate".to_string(),
        node_url: "wss://rococo-rpc.polkadot.io".to_string(),
        event_filter: "balances=Transfer;system".to_string(),
        contract_filter: "".to_string(),
    };
    let polkadot = PolkadotFeed::<PolkadotConfig>::new(config).await;
    let result = polkadot.split_filter();

    assert_eq!(
        result,
        std::collections::HashMap::from([
            ("balances".to_string(), vec!["Transfer"]),
            ("system".to_string(), vec![])
        ])
    );
}
