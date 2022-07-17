mod prclient;
mod types;

use clap;
use prclient::PRClient;
use reqwest;
use serde_json;
use std::fs;
use std::io;
use tokio;
use types::*;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
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
