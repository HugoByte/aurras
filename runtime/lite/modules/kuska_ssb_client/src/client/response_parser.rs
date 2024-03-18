use super::*;

pub fn whoami_res_parse(body: &[u8]) -> Result<WhoAmIOut> {
    Ok(serde_json::from_slice(body)?)
}
pub fn message_res_parse(body: &[u8]) -> Result<Message> {
    Ok(Message::from_slice(body)?)
}
pub fn feed_res_parse(body: &[u8]) -> Result<Feed> {
    Ok(Feed::from_slice(&body)?)
}

pub fn latest_res_parse(body: &[u8]) -> Result<LatestOut> {
    Ok(serde_json::from_slice(body)?)
}

pub fn invite_create(body: &[u8]) -> Result<String> {
    Ok(serde_json::from_slice(body)?)
}
