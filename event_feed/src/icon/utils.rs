use super::*;
use runtime::{logger::CoreLogger, Logger};

pub async fn get_icon_block_height(icon_service: &IconService) -> Result<usize> {
    let logger = CoreLogger::new(Some("./event-feed.log"));
    let latest_block = icon_service
        .get_last_block()
        .await
        .unwrap()
        .get("result")
        .and_then(|value| value.get("height"))
        .unwrap()
        .clone();
    logger.info(&format!("Got the latest block height {:?}", latest_block));

    Ok(serde_json::from_value(latest_block)?)
}

pub async fn get_event_logs_by_tx_hash(
    icon_service: &IconService,
    tx_hash: &str,
) -> Result<Vec<TxEventLog>> {
    let logger = CoreLogger::new(Some("./event-feed.log"));
    let mut tx_result = icon_service.get_transaction_result(tx_hash).await;

    while let Err(error) = tx_result {
        if error
            .to_string()
            .contains(r#"\"error\":{\"code\":-31003,\"message\":\"Executing: Executing\"}"#)
        {
            return Err(anyhow!(
                "error in getting transaction result for tx_hash:{tx_hash:?} error:{error:?}"
            ));
        }

        logger.info(&format!("Retrying the transaction result fetch for tx_hash: {}", tx_hash));
        sleep(Duration::from_millis(1000));
        tx_result = icon_service.get_transaction_result(tx_hash).await
    }

    let event_logs: Vec<TxEventLog> = serde_json::from_value(
        tx_result
            .unwrap()
            .get("result")
            .and_then(|result| result.get("eventLogs"))
            .unwrap()
            .clone(),
    )
    .unwrap();
    logger.info(&format!("Fetched {} event logs for the tx_hash {}", event_logs.len(), tx_hash));

    Ok(event_logs)
}
