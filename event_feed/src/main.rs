use event_feed::{
    ChainConfig, Context, CosmosFeed, EthFeed, IOError, IconFeed, PolkadotFeed, Producer,
    ProducerConfig,
};
use std::{fs, path};
use subxt::PolkadotConfig;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Async CLI Example")]
struct EventFeed {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, global = true, long, help = "Optional path to configuration")]
    pub config: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "substrate")]
    Substrate,
    #[command(about = "Cosmos")]
    Cosmos,
    #[command(about = "Icon")]
    Icon,
    #[command(about = "Eth")]
    Eth,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = EventFeed::parse();

    //TODO config none case
    let (chain_config, producer_config) = match args.clone().config {
        Some(path) => {
            let read_data = fs::read(path::Path::new(&path))?;
            let chain_config: ChainConfig =
                serde_json::from_slice(&read_data).map_err(|e| IOError::Anyhow(e.into()))?;
            let producer_config: ProducerConfig =
                serde_json::from_slice(&read_data.clone()).unwrap();
            (chain_config, producer_config)
        }
        None => (ChainConfig::default(), ProducerConfig::default()),
    };
    let mut ssb_client = Producer::new(producer_config.clone())
        .await
        .map_err(|e| IOError::Anyhow(e.into()))?;
    ssb_client
        .accept_invite()
        .await
        .map_err(|e| IOError::Anyhow(e.into()))?;

    let feed = match args.command {
        Commands::Substrate => {
            let polkadot_client = PolkadotFeed::<PolkadotConfig>::new(chain_config.clone())
                .await
                .map_err(|e| {
                    IOError::Other(format!("Error initializing POLKADOT client : {}", e))
                })?;
            Context::PolkadotFeed(polkadot_client)
        }
        Commands::Icon => {
            let icon_client = IconFeed::new(chain_config.clone())
                .map_err(|e| IOError::Other(format!("Error initializing ICON client: {}", e)))?;
            Context::IconFeed(icon_client)
        }
        Commands::Eth => {
            let eth_client = EthFeed::new(chain_config.clone())
                .await
                .map_err(|e| IOError::Other(format!("Error initializing ETH client: {}", e)))?;
            Context::EthFeed(eth_client)
        }
        Commands::Cosmos => {
            let cosmos_client = CosmosFeed::new(chain_config.clone());
            Context::CosmosFeed(cosmos_client)
        }
    };

    let (tx, rx) = std::sync::mpsc::channel::<Vec<serde_json::Value>>();

    tokio::spawn(async move {
        while let Ok(logs) = rx.recv() {
            for log in logs {
                ssb_client.publish_feed(log).await.unwrap();
            }
        }
    });

    let _ = feed
        .feed_events(&|e| {
            tx.send(e).unwrap();
        })
        .await;

    Ok(())
}
