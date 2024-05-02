use super::*;
use anyhow::anyhow;
use anyhow::Result;
use runtime::api::dto::content::Mention;
use runtime::kuska_ssb_client::client::owned_identity_from_patchwork_config;
use runtime::kuska_ssb_client::client::Client;

/// Represents the aurras ssb producer client.
pub struct Producer {
    config: ProducerConfig,
    client: Client,
}

impl Producer {

    /// Creates a new ssb producer client.
    /// 
    /// # Arguments
    /// 
    /// * `config` - A configuration struct for the ssb producer.
    /// 
    /// # Returns
    /// 
    /// Returns a new ssb producer client.
    pub async fn new(config: ProducerConfig) -> Result<Self> {
        let key = owned_identity_from_patchwork_config(&config.ssb_secret)
            .await
            .map_err(|e| anyhow!("error while creating identity: {e:?}"))?;
        let client = Client::new(Some(&key), &config.ssb_ip, &config.ssb_port)
            .await
            .map_err(|e| anyhow!("error while creating producer client: {e:?}"))?;

        Ok(Self { config, client })
    }

    /// Accepts an ssb pub invite.
    pub async fn accept_invite(&mut self) -> Result<()> {
        if let Err(e) = self.client.accept_invite(&self.config.ssb_invite).await {
            let error = format!("{e:?}");

            if !error.contains("already following") {
                return Err(anyhow!(error));
            }
        };

        Ok(())
    }

    /// Publishes the event feed to the pub.
    /// 
    /// # Arguments
    /// 
    /// * `message` - The event feed message to publish.
    pub async fn publish_feed(&mut self, message: Value) -> Result<()> {
        let mention = Mention {
            link: get_ssb_pub_address_from_invite(self.config.ssb_invite.as_str()),
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

/// Returns the ssb pub address from the invite.
pub fn get_ssb_pub_address_from_invite(invite: &str) -> String {
    let pub_address = invite
        .split('@')
        .skip(1)
        .next()
        .unwrap()
        .split("=.ed25519")
        .next()
        .unwrap();

    format!("@{pub_address}=.ed25519")
}
