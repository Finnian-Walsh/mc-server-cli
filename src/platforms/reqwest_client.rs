use crate::error::Result;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use std::sync::OnceLock;

const CONTACT_RAW: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/contact"));

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn create() -> Result<&'static Client> {
    if let Some(client) = CLIENT.get() {
        return Ok(client);
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&format!(
            "{}/{} (contact: {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            CONTACT_RAW.trim_start().trim_end().to_string()
        ))?,
    );

    let client = Client::builder().default_headers(headers).build()?;

    Ok(CLIENT.get_or_init(|| client))
}
