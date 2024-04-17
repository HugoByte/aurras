use super::*;

pub struct IconFeed {
    producer_client: Producer,
    icon_service: IconService,
    logger: Box<dyn Logger>,
}

impl IconFeed {
    pub async fn new(
        icon_api_endpoint: &str,
        producer_client: Producer,
        logger: Box<dyn Logger>,
    ) -> Result<Self> {
        let icon_service = icon_service::IconService::new(Some(icon_api_endpoint.to_string()));

        Ok(Self {
            producer_client,
            icon_service,
            logger,
        })
    }

    pub async fn get_event(&mut self, filter: Filter, publish_to: &str) -> Result<()> {
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

                for transaction in transactions {
                    if filter.filter(&self.icon_service, &transaction).await? {
                        self.logger.info("ICON event found!");
                        self.producer_client
                            .publish_feed(publish_to, transaction)
                            .await?;
                        self.logger.info("Event published to the pub");
                    }
                }

                old_height += 1;
            } else {
                sleep(Duration::from_secs(1));
            }

            latest_height = get_icon_block_height(&self.icon_service).await?;
        }
    }
}
