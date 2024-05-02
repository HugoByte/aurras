// use crate::common::Context;
// use crate::common::Producer;
// use common::cosmos::CosmosFeed;
// use common::{ChainConfig, ProducerConfig};
// use eth::EthFeed;
// use icon::IconFeed;
// use substrate::feeder::PolkadotFeed;
// use subxt::PolkadotConfig;

// pub mod common;
// mod cosmos;
// mod eth;
// mod icon;
// mod substrate;
// // pub use cosmos;

// #[tokio::main]
// async fn main() {
//     dotenv::dotenv().ok();
//     let chain_config = envy::from_env::<ChainConfig>().unwrap();
//     let producer_config = envy::from_env::<ProducerConfig>().unwrap();

//     let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
//     ssb_client.accept_invite().await.unwrap();

//     let feed: Context = match chain_config.clone().chain.to_lowercase().as_str() {
//         "substrate" => {
//             let polkadot_client = PolkadotFeed::<PolkadotConfig>::new(chain_config.clone())
//                 .await
//                 .unwrap();
//             Context::PolkadotFeed(polkadot_client)
//         }
//         "icon" => {
//             let icon_client = IconFeed::new(chain_config.clone()).unwrap();
//             Context::IconFeed(icon_client)
//         }
//         "eth" => {
//             let eth_client = EthFeed::new(chain_config.clone()).await.unwrap();
//             Context::EthFeed(eth_client)
//         }
//         "cosmos" => {
//             let cosmos_client = CosmosFeed::new(chain_config.clone());
//             Context::CosmosFeed(cosmos_client)
//         }
//         _ => panic!("Invalid Chain"),
//     };

//     let _ = feed
//         .feed_events(&|e| {
//             for i in e {
//                 tokio::spawn(async move {
//                     let producer_config = envy::from_env::<ProducerConfig>().unwrap();
//                     let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
//                     ssb_client
//                         .publish_feed(i)
//                         .await
//                         .expect("Failed to send event");
//                 });
//             }
//         })
//         .await;
// }

use event_feed::{
    ChainConfig, Context, CosmosFeed, EthFeed, IconFeed, PolkadotFeed, Producer, ProducerConfig,
};
use std::{
    fs,
    path::{self, PathBuf},
};
use subxt::PolkadotConfig;

use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Async CLI Example")]
struct EventFeed {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, global = true, long, help = "Optional path to configuration")]
    pub config: Option<PathBuf>,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = EventFeed::parse();

    // let chain_config = envy::from_env::<ChainConfig>().unwrap();
    let read_data = fs::read(path::Path::new(&args.clone().config.unwrap()))?;
    let chain_config: ChainConfig = serde_json::from_slice(&read_data).unwrap();
    let producer_config: ProducerConfig = serde_json::from_slice(&read_data.clone()).unwrap();

    let rt = Runtime::new()?;

    let feed = match args.command {
        Commands::Substrate => {
            let polkadot_client = rt
                .block_on(PolkadotFeed::<PolkadotConfig>::new(chain_config.clone()))
                .unwrap();
            Context::PolkadotFeed(polkadot_client)
        }
        Commands::Icon => {
            let icon_client = IconFeed::new(chain_config.clone()).unwrap();
            Context::IconFeed(icon_client)
        }
        Commands::Eth => {
            let eth_client = rt.block_on(EthFeed::new(chain_config.clone())).unwrap();
            Context::EthFeed(eth_client)
        }
        Commands::Cosmos => {
            let cosmos_client = CosmosFeed::new(chain_config.clone());
            Context::CosmosFeed(cosmos_client)
        }
    };

    let _ = rt.block_on(feed.feed_events(&|e| {
        for i in e {
            tokio::spawn(async {
                let args = EventFeed::parse();
                let read_data = fs::read(path::Path::new(&args.config.unwrap())).unwrap();
                let producer_config: ProducerConfig =
                    serde_json::from_slice(read_data.clone().as_slice()).unwrap();
                let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
                ssb_client
                    .publish_feed(i)
                    .await
                    .expect("Failed to send event");
            });
        }
    }));

    Ok(())
}
