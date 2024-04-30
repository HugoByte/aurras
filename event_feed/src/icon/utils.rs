use super::*;

/// The function `get_icon_block_height` retrieves the height of the latest block from an IconService
/// asynchronously in Rust.
/// 
/// Arguments:
/// 
/// * `icon_service`: The `icon_service` parameter is of type `&IconService`, which is a reference to an
/// instance of the `IconService` struct. This struct likely provides functionality to interact with the
/// ICON blockchain network, such as fetching block information, transactions, etc.
/// 
/// Returns:
/// 
/// The function `get_icon_block_height` is returning a `Result` containing a `usize` value.
pub async fn get_icon_block_height(icon_service: &IconService) -> Result<usize> {
    let latest_block = icon_service
        .get_last_block()
        .await
        .unwrap()
        .get("result")
        .and_then(|value| value.get("height"))
        .unwrap()
        .clone();

    Ok(serde_json::from_value(latest_block)?)
}

/// This Rust function retrieves event logs associated with a transaction hash using the IconService.
/// 
/// Arguments:
/// 
/// * `icon_service`: The `icon_service` parameter in the `get_event_logs_by_tx_hash` function is an
/// instance of the `IconService` struct or class. It is used to interact with the ICON blockchain
/// network to retrieve transaction results and event logs related to a specific transaction hash. This
/// parameter allows the function to
/// * `tx_hash`: The `tx_hash` parameter in the `get_event_logs_by_tx_hash` function is a string that
/// represents the transaction hash for which you want to retrieve the event logs. This function uses an
/// `IconService` instance to fetch the transaction result associated with the provided transaction hash
/// and then extracts and returns
/// 
/// Returns:
/// 
/// The function `get_event_logs_by_tx_hash` returns a `Result` containing a vector of `TxEventLog`
/// structs.
pub async fn get_event_logs_by_tx_hash(
    icon_service: &IconService,
    tx_hash: &str,
) -> Result<Vec<TxEventLog>> {
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

    Ok(event_logs)
}
