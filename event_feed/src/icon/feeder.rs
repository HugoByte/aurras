use super::*;

pub struct IconFeed {
    icon_service: IconService,
    filter: Filter,
    logger: Box<dyn Logger>,
}

impl IconFeed {
    pub fn new(config: ChainConfig) -> Result<IconFeed> {
        let events = config
            .event_filter
            .split(',')
            .map(|evt| evt.to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let score = config
            .contract_filter
            .split(',')
            .map(|evt| evt.to_string())
            .filter(|evt| !evt.is_empty())
            .collect::<Vec<String>>();

        let logger = CoreLogger::new(Some("./icon-feed.log"));

        logger.info("Connecting to producer...");
        logger.info("Producer successfully connected✅");

        let icon_service = icon_service::IconService::new(Some(config.node_url));

        let filter = Filter { events, score };

        let icon_feed = IconFeed {
            icon_service,
            filter,
            logger: Box::new(logger),
        };

        Ok(icon_feed)
    }

    pub async fn event_feed(&self, cb: fn(events: Vec<serde_json::Value>)) -> Result<()> {
        self.logger.info("Listening to icon chain for events...");

        let mut latest_height = get_icon_block_height(&self.icon_service).await?;
        let mut old_height = latest_height - 1;

        loop {
            if old_height < latest_height {
                let block = match self
                    .icon_service
                    .get_block_by_height(&format!("0x{:x}", latest_height))
                    .await
                {
                    Ok(block) => block,
                    Err(_) => {
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                };

                let transactions: Vec<Value> = serde_json::from_value(
                    block
                        .get("result")
                        .and_then(|val| val.get("confirmed_transaction_list"))
                        .unwrap()
                        .clone(),
                )?;

                let mut filtered_tx = Vec::<Value>::new();

                for transaction in transactions {
                    if self.filter.filter(&self.icon_service, &transaction).await? {
                        self.logger.info("ICON event found!");
                        filtered_tx.push(transaction);
                    }
                }

                if !filtered_tx.is_empty() {
                    cb(filtered_tx)
                }

                old_height += 1;
            } else {
                sleep(Duration::from_secs(1));
            }

            latest_height = get_icon_block_height(&self.icon_service).await?;
        }
    }
}
