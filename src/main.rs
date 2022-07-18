/// A PRClient stands for PigRabbitClient is technically
/// a reqwest client with authentication keys that was required by PorkBun API
///
/// Therefore, to construct the client, the apikey and secretapikey should be provided
/// to the PRClient in the Keys struct format before performing any interaction to the API.
///
/// An usage example of the PRClient:
/// ```
/// let keys_file: String = fs::read_to_string("keys.json").expect("File not found!");
/// let keys: Keys = serde_json::from_str(&);
/// let mut client = PRClient::new(keys);
///
/// client.retrieve_by_domain_with_id("example.com", "1234567").await;
/// client.del_by_id("example.com","1234567").await;
/// ```
mod prclient;
mod types;

use prclient::PRClient;
use serde_json;
use std::fs;
use tokio;
use types::*;

#[tokio::main]
async fn main() {
    let config_file = fs::read_to_string("config.json").expect("[Pigrabbit] File does not exist!");
    let key_struct: Keys = serde_json::from_str(&config_file).unwrap();
    let mut prclient = PRClient::new(key_struct);
    prclient
        .retreive_by_domain_with_id(
            // "A".to_owned(),
            "pornhub.com",
            "", // None,
        )
        .await;
}
