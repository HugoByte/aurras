#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    // use super::*;
    use crate::{Client, Event, UserConfig};

    // ssb-server should keep running for testing
    /* configure the env variables such as SSB_IP, SSB_PORT, SSB_PUBLIC_KEY,
    SSB_SECRET_KEY and SSB_ID in .env file */
    // use `cargo test -- --ignored` command for testing

    #[async_std::test]
    #[ignore]
    async fn test_client() {
        dotenv().ok();

        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();

        Client::new(None, ssb_ip, ssb_port).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_client_with_config() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        // passing default ip and port of ssb-server for testing
        Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
    }

    #[async_std::test]
    #[should_panic = "fail to create client"]
    #[ignore]
    async fn test_client_with_config_fail() {
        let config = UserConfig::new("public key", "private key", "address");

        // passing default ip and port of ssb-server for testing
        Client::new(Some(config), "".to_string(), "".to_string())
            .await
            .expect("fail to create client");
    }

    #[async_std::test]
    #[ignore]
    async fn test_get_secret_key() {
        use kuska_ssb::crypto::ed25519::SecretKey;

        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        let secret_key = client.get_secret_key();

        let secret_key_config =
            SecretKey::from_slice(&base64::decode(&ssb_secret_key).unwrap()).unwrap();
        assert_eq!(secret_key, secret_key_config);
    }

    #[async_std::test]
    #[ignore]
    async fn test_whoami() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        let whoami = client.whoami().await.unwrap();
        assert_eq!(whoami, ssb_id);
    }

    #[async_std::test]
    #[ignore]
    async fn test_get_method(){
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);
        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
        // make sure the feed is not empty
        client.publish("hello world", None).await.unwrap();
        // wait for server to publish
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;

        let feed = client.user(false , "me").await.unwrap();
        let feed_by_id = client.get(&feed[feed.len()-1].key).await.unwrap().value;
        let feed = serde_json::to_value(&feed.last().unwrap().value).unwrap();

        assert_eq!(feed, feed_by_id);        
    }

    #[async_std::test]
    #[ignore]
    // returns list of feeds posted by particular user
    async fn test_user_method() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        let old_event = Event {
            id: "1".to_string(),
            body: "hello_world_event".to_string(),
        };

        let value = serde_json::to_value(old_event.clone()).unwrap();

        client.publish(&value.to_string(), None).await.unwrap();

        // wait for server to publish
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;

        let feed = client.user(false, &ssb_id).await.unwrap();

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
    // returns list of feeds posted by particular user
    async fn test_user_me() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        let old_event = Event {
            id: "1".to_string(),
            body: "hello_world_event".to_string(),
        };

        let value = serde_json::to_value(old_event.clone()).unwrap();

        client.publish(&value.to_string(), None).await.unwrap();

        // wait for server to publish
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;

        let feed = client.user(false, "me").await.unwrap();

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
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        client.close().await.unwrap();
        client.whoami().await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_feed() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();

        client.feed(false).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_publish() {
        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

        let mut client = Client::new(Some(config), ssb_ip, ssb_port).await.unwrap();
        let feed = client.feed(false).await.unwrap();
        let prev_len = feed.len();

        let old_event = Event {
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

        let new_event: Event = serde_json::from_str(&feed_text).unwrap();
        // let event = serde_json::from_value(event).unwrap();
        assert_eq!(old_event, new_event);
    }

    #[tokio::test]
    #[ignore]
    async fn test_event_subscription() {
        use crate::{Client, UserConfig};

        dotenv().ok();
        let ssb_ip = std::env::var("SSB_IP").unwrap();
        let ssb_port = std::env::var("SSB_PORT").unwrap();
        let ssb_public_key = std::env::var("SSB_PUBLIC_KEY").unwrap();
        let ssb_secret_key = std::env::var("SSB_SECRET_KEY").unwrap();
        let ssb_id = std::env::var("SSB_ID").unwrap();

        let config = UserConfig::new(&ssb_public_key, &ssb_secret_key, &ssb_id);

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

        #[subxt::subxt(runtime_metadata_path = "../utils/polkadot_metadata_small.scale")]
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
