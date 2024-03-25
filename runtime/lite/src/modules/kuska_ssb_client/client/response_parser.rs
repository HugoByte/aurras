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
    Ok(std::str::from_utf8(&body)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
        .to_string())
}

pub fn invite_accept_res_parse(body: &[u8]) -> Result<Vec<Feed>> {
    Ok(serde_json::from_slice(body)?)
}
