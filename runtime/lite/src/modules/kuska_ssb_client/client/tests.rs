#[cfg(test)]
mod tests {
    use crate::modules::kuska_ssb_client::client::{types, Client, UserConfig};
    use dotenv::dotenv;
    use kuska_ssb::keystore::read_patchwork_config;

    // ssb-server should keep running for testing
    /* configure the env variables such as ssb-sercret file path, ip and port where
    ssb-server is running in .env file */
    // use `cargo test -- --ignored` command for testing

    #[async_std::test]
    #[ignore]
    async fn test_client() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_client_with_config() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        // passing default ip and port of ssb-server for testing
        Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_get_secret_key() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let client = Client::new(Some(config.clone()), ssb_ip, ssb_port)
            .await
            .unwrap();

        let secret_key = client.get_secret_key();

        assert_eq!(secret_key, config.sk);
    }

    #[async_std::test]
    #[ignore]
    async fn test_whoami() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let mut client = Client::new(Some(config.clone()), ssb_ip, ssb_port)
            .await
            .unwrap();

        let whoami = client.whoami().await.unwrap();
        assert_eq!(whoami, config.id);
    }

    #[async_std::test]
    #[ignore]
    // returns list of feeds posted by particular user
    async fn test_user_method() {
        use types::Event;
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let mut client = Client::new(Some(config.clone()), ssb_ip, ssb_port)
            .await
            .unwrap();

        let old_event = Event {
            id: "1".to_string(),
            body: "hello_world_event".to_string(),
        };

        let value = serde_json::to_value(old_event.clone()).unwrap();

        client.publish(&value.to_string(), None).await.unwrap();

        // wait for server to publish
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;

        let feed = client.user(false, &config.id).await.unwrap();

        let event = feed.last().unwrap().value.clone();
        let message = event.get("content").unwrap();

        let feed_type = message.get("type").unwrap();
        let feed_type: String = serde_json::from_value(feed_type.clone()).unwrap();

        assert_eq!(&feed_type, "post");

        let feed_text = message.get("text").unwrap();
        let feed_text: String = serde_json::from_value(feed_text.clone()).unwrap();

        let new_event: Event = serde_json::from_str(&feed_text).unwrap();
        // let event = serde_json::from_value(event).unwrap();
        assert_eq!(old_event, new_event);
    }

    #[async_std::test]
    #[ignore]
    #[should_panic = "Already closed"]
    async fn test_close() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        client.close().await.unwrap();
        client.whoami().await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_feed() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        client.feed(false).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_feed_test() {
        use crate::modules::kuska_ssb_client::client::UserConfig;
        let user = UserConfig::new("vhuaeBySHfMTeBpTseKP/ksOVtyLGaqZ+Ae4SyQk7wY=", 
    "MywOEUUCk9rUcWq6OFsfbzZABDc+sItJHJoN+RJrwMK+G5p4HJId8xN4GlOx4o/+Sw5W3IsZqpn4B7hLJCTvBg=", 
    "@vhuaeBySHfMTeBpTseKP/ksOVtyLGaqZ+Ae4SyQk7wY=.ed25519");

        let mut client = Client::new(None, "0.0.0.0".to_string(), "8015".to_string())
            .await
            .unwrap();
        client.feed(true).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_publish() {
        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
        let feed = client.feed(false).await.unwrap();
        let prev_len = feed.len();

        let old_event = types::Event {
            id: "1".to_string(),
            body: "hello_world_event".to_string(),
        };

        let value = serde_json::to_value(old_event.clone()).unwrap();

        let result = client.publish(&value.to_string(), None).await;
        assert!(result.is_ok());

        // wait for server to publish
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        let feed = client.feed(false).await.unwrap();
        assert!(feed.len() > prev_len);

        let event = feed.last().unwrap().value.clone();
        let message = event.get("content").unwrap();

        let feed_type = message.get("type").unwrap();
        let feed_type: String = serde_json::from_value(feed_type.clone()).unwrap();

        assert_eq!(&feed_type, "post");

        let feed_text = message.get("text").unwrap();
        let feed_text: String = serde_json::from_value(feed_text.clone()).unwrap();

        let new_event: types::Event = serde_json::from_str(&feed_text).unwrap();
        assert_eq!(old_event, new_event);
    }

    #[tokio::test]
    #[ignore]
    async fn test_event_subscription() {
        use super::*;

        dotenv().ok();

        let secret = std::env::var("SECRET").unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().unwrap();
            std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
        });
        let ssb_port = std::env::var("PORT").unwrap_or_else(|_| 8008.to_string());
        let ssb_ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
        let mut file = async_std::fs::File::open(secret).await.unwrap();
        let config = read_patchwork_config(&mut file).await.unwrap();
        //TODO
        // Must start a local dev polkadot Node
        // Must start and setup a ssb-server
        // Use the script to start the ssb-server
        // Copy the secret file and setup here for client

        // Tranfer the amount manually after starting this function

        //Todo
        // Change user configuration
        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        use subxt::{OnlineClient, PolkadotConfig};
        use subxt_signer::sr25519::dev;

        #[subxt::subxt(runtime_metadata_path = "./src/modules/utils/polkadot_metadata_small.scale")]
        pub mod polkadot {}

        let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

        // Subscribe to all finalized blocks:
        let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

        // For each block, print a bunch of information about it:
        'outer: while let Some(block) = blocks_sub.next().await {
            let block = block.unwrap();

            let block_number = block.header().number;
            let block_hash = block.hash();

            println!("Block #{block_number}:");
            println!("  Hash: {block_hash}");
            println!("  Extrinsics:");

            if block_number == 10 {
                let dest = dev::bob().public_key().into();
                let tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);
                let from = dev::alice();
                let _events = api
                    .tx()
                    .sign_and_submit_then_watch_default(&tx, &from)
                    .await
                    .unwrap();
            }

            // Log each of the extrinsic with it's associated events:
            let extrinsics = block.extrinsics().await.unwrap();
            for ext in extrinsics.iter() {
                let ext = ext.unwrap();
                let events = ext.events().await.unwrap();
                let transfer = events
                    .find_first::<polkadot::balances::events::Transfer>()
                    .unwrap();

                match transfer {
                    Some(transfer) => {
                        let from_addr = transfer.from.to_string();
                        let to_addr = transfer.from.to_string();
                        let amount = transfer.amount;
                        println!("{from_addr:?}");

                        let value = format!(
                            "{{\"from\":\"{}\",\"to\":\"{}\",\"amount\":\"{}\"}}",
                            from_addr, to_addr, amount
                        );

                        let result = client.publish(&value, None).await;
                        assert!(result.is_ok());
                        break 'outer;
                    }
                    None => (),
                }
            }
        }
    }
}
