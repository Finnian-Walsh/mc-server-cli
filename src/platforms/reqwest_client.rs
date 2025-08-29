use crate::error::Result;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use std::sync::OnceLock;

const CONTACT_RAW: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/contact"));

static USER_AGENT_STRING: OnceLock<String> = OnceLock::new();

pub fn get_user_agent_str() -> &'static str {
    USER_AGENT_STRING.get_or_init(|| {
        format!(
            "{}/{} (contact: {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            CONTACT_RAW.trim_start().trim_end().to_string()
        )
    })
}

pub fn get() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(get_user_agent_str())?);

    Ok(Client::builder().default_headers(headers).build()?)
}
