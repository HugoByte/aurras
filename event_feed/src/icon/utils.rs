use super::*;

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

pub fn get_list_env_var(var: &str) -> Option<Vec<String>> {
    match env::var(var) {
        Ok(events) => Some(
            events
                .split(',')
                .map(|evt| evt.to_string())
                .filter(|evt| !evt.is_empty())
                .collect::<Vec<String>>(),
        ),
        Err(_) => None,
    }
}

pub fn get_ssb_pub_address_from_invite(invite: &str) -> Result<String> {
    let pub_address = invite
        .split('@')
        .skip(1)
        .next()
        .unwrap()
        .split("=.ed25519")
        .next()
        .unwrap();

    Ok(format!("@{pub_address}=.ed25519"))
}
