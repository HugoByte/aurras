use super::*;

pub struct Producer {
    client: Client,
}

impl Producer {
    pub async fn create_ssb_producer(
        ssb_ip: Option<&str>,
        ssb_port: Option<&str>,
        ssb_secret: Option<&str>,
    ) -> Result<Self> {
        let secret = match ssb_secret {
            Some(secret) => secret.to_string(),
            None => {
                let home_dir = dirs::home_dir().unwrap();
                format!("{}/.ssb/secret", home_dir.to_string_lossy())
            }
        };

        let port = ssb_port.unwrap_or(DEFAULT_PRODUCER_PORT);
        let ip = ssb_ip.unwrap_or(DEFAULT_PRODUCER_IP);

        let mut file = async_std::fs::File::open(secret).await?;
        let key = read_patchwork_config(&mut file).await?;
        let client = Client::new(Some(key), ip.to_string(), port.to_string())
            .await
            .map_err(|e| anyhow!("error while creating producer client: {e:?}"))?;

        Ok(Self { client })
    }

    pub async fn accept_invite(&mut self, pub_address: &str) -> Result<()> {
        if let Err(e) = self.client.accept_invite(&pub_address).await {
            let error = format!("{e:?}");

            if !error.contains("already following") {
                return Err(anyhow!(error));
            }
        };

        Ok(())
    }

    pub async fn publish_feed(&mut self, pub_address: &str, message: Value) -> Result<()> {
        let mention = Mention {
            link: pub_address.to_string(),
            name: None,
        };

        let result = self
            .client
            .publish(&message.to_string(), Some(vec![mention]))
            .await;

        result.unwrap();

        Ok(())
    }
}

