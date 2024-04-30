use super::*;

/// Parses the type `[u8]` to a `WhoAmIOut` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<WhoAmIOut>` containing a `WhoAmIOut` struct or an error.
pub fn whoami_res_parse(body: &[u8]) -> Result<WhoAmIOut> {
    Ok(serde_json::from_slice(body)?)
}

/// Parses the type `[u8]` to a `Message` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<Message>` containing a `Message` struct or an error.
pub fn message_res_parse(body: &[u8]) -> Result<Message> {
    Ok(Message::from_slice(body)?)
}

/// Parses the type `[u8]` to a `Feed` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<Feed>` containing a `Feed` struct or an error.
pub fn feed_res_parse(body: &[u8]) -> Result<Feed> {
    Ok(Feed::from_slice(body)?)
}

/// Parses the type `[u8]` to a `LatestOut` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<LatestOut>` containing a `LatestOut` struct or an error.
pub fn latest_res_parse(body: &[u8]) -> Result<LatestOut> {
    Ok(serde_json::from_slice(body)?)
}

/// Parses the type `[u8]` to a `CreateStreamIn` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<CreateStreamIn>` containing a `CreateStreamIn` struct or an error.
pub fn invite_create(body: &[u8]) -> Result<String> {
    Ok(std::str::from_utf8(body)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
        .to_string())
}

/// Parses the type `[u8]` to a `Vec<Feed>` struct.
/// 
/// # Arguments
/// 
/// * `body`: Reference to a byte slice (`&[u8]`).
/// 
/// # Returns
/// 
/// Returns a `Result<Vec<Feed>>` containing a `Vec<Feed>` struct or an error.
pub fn invite_accept_res_parse(body: &[u8]) -> Result<Vec<Feed>> {
    Ok(serde_json::from_slice(body)?)
}
