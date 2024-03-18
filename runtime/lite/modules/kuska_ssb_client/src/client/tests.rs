#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Client, Event};

    // ssb-server should keep running for testing
    // use `cargo test -- --ignored` command for testing
    #[async_std::test]
    #[ignore]
    async fn test_client() {
        // passing default ip and port of ssb-server for testing
        let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
            .await
            .unwrap();
        client.user(false, "me").await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_feed() {
        let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
            .await
            .unwrap();
        client.feed(false).await.unwrap();
    }

    #[async_std::test]
    #[ignore]
    async fn test_publish() {
        let mut client = Client::new(None, "0.0.0.0".to_string(), "8008".to_string())
            .await
            .unwrap();
        let feed = client.feed(false).await.unwrap();
        let prev_len = feed.len();

        let old_event = Event {
            id: "1".to_string(),
            body: "hello_world_event".to_string(),
        };

        let value = serde_json::to_value(old_event.clone()).unwrap();

        let result = client.publish(&value.to_string()).await;
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
}
