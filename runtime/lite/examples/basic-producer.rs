use dotenv::dotenv;
use kuska_ssb::{api::dto::content::Mention, keystore::read_patchwork_config};
use runtime::kuska_ssb_client::client::Client;
use runtime::logger::{CoreLogger, Logger};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let logger = CoreLogger::new(Some("./ssb-producer.log"));

    logger.info("starting producer...");
    let secret = std::env::var("PRODUCER_SECRET").unwrap_or_else(|_| {
        let home_dir = dirs::home_dir().unwrap();
        std::format!("{}/.ssb/secret", home_dir.to_string_lossy())
    });

    let port = std::env::var("PRODUCER_PORT").unwrap_or("8014".to_string());
    let pub_address = std::env::var("PUB_ADDRESS").expect("Pub address must be provided");

    let mut file = async_std::fs::File::open(secret).await.unwrap();
    let key = read_patchwork_config(&mut file).await.unwrap();

    let mut client = Client::new(Some(key), "0.0.0.0".to_string(), port)
        .await
        .unwrap();

    logger.info("producer successfully started✅");

    use subxt::{OnlineClient, PolkadotConfig};

    #[subxt::subxt(runtime_metadata_path = "./src/modules/utils/polkadot_metadata_small.scale")]
    pub mod polkadot {}

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    // For each block, print a bunch of information about it:
    while let Some(block) = blocks_sub.next().await {
        let block = block.unwrap();

        let block_number = block.header().number;
        let block_hash = block.hash();

        logger.info(&format!(
            "Block #{block_number} Hash: {block_hash} Extrinsics:"
        ));

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

                    logger.info(&format!("Transfer: {from_addr} -> {to_addr} {amount}"));

                    let value = format!(
                        "{{\"from\":\"{}\",\"to\":\"{}\",\"amount\":\"{}\"}}",
                        from_addr, to_addr, amount
                    );

                    let mention = Mention {
                        link: pub_address.clone(),
                        name: None,
                    };

                    let _ = client
                        .publish(&value.to_string(), Some(vec![mention]))
                        .await;
                }
                None => (),
            }
        }
    }
}
