pub mod common;
pub mod cosmos;
pub mod error;
pub mod eth;
pub mod icon;
pub mod substrate;

/// can use this library to fetch events from multiple chain like
/// - substrate
/// - icon
/// - eth
/// - cosmos
///
/// use event_feed::*;
///
/// #[tokio::main]
/// async fn main() {
///     dotenv::dotenv().ok();
///     let chain_config = envy::from_env::<ChainConfig>().unwrap();
///     let producer_config = envy::from_env::<ProducerConfig>().unwrap();

///     let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
///     ssb_client.accept_invite().await.unwrap();

///     let feed: Context = match chain_config.clone().chain.to_lowercase().as_str() {
///         "substrate" => {
///             let polkadot_client = PolkadotFeed::<PolkadotConfig>::new(chain_config.clone())
///                 .await
///                 .unwrap();
///             Context::PolkadotFeed(polkadot_client)
///         }
///         "icon" => {
///             let icon_client = IconFeed::new(chain_config.clone()).unwrap();
///             Context::IconFeed(icon_client)
///         }
///         "eth" => {
///             let eth_client = EthFeed::new(chain_config.clone()).await.unwrap();
///             Context::EthFeed(eth_client)
///         }
///         "cosmos" => {
///             let cosmos_client = CosmosFeed::new(chain_config.clone());
///             Context::CosmosFeed(cosmos_client)
///         }
///         _ => panic!("Invalid Chain"),
///     };

///     let _ = feed
///         .feed_events(&|e| {
///             for i in e {
///                 tokio::spawn(async move {
///                     let producer_config = envy::from_env::<ProducerConfig>().unwrap();
///                     let mut ssb_client = Producer::new(producer_config.clone()).await.unwrap();
///                     ssb_client
///                         .publish_feed(i)
///                         .await
///                         .expect("Failed to send event");
///                 });
///             }
///         })
///         .await;
/// }
///
pub use common::*;
pub use cosmos::*;
pub use error::*;
pub use eth::*;
pub use icon::*;
pub use substrate::*;
