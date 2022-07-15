use clap;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use tokio;

#[derive(Serialize, Deserialize)]
struct ComplicatedBody {
    secretapikey: String,
    apikey: String,
    name: String,
    r#type: String,
    content: String,
    ttl: i32,
}

struct SimpleBody {
    secretapikey: String,
    apikey: String,
    content: String,
    ttl: i32,
}

struct Keys {
    secretapikey: String,
    apikey: String,
}

struct RecordVar {
    name: String,
    r#type: String,
    content: String,
    ttl: i32,
}

async fn addRecord(
    client: reqwest::Client,
    domain_name: String,
    keys: Keys,
    record_struct: RecordVar,
) -> Option<()> {
    let url = "https://porkbun.com/api/json/v3/dns/create/".to_owned() + &domain_name;
    let body = ComplicatedBody {
        secretapikey: keys.secretapikey,
        apikey: keys.apikey,
        name: record_struct.name,
        r#type: record_struct.r#type,
        content: record_struct.content,
        ttl: record_struct.ttl,
    };
    let res: serde_json::Value = client
        .post(url)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .expect("The request received was not in the JSON format omg!!!! pls contact devs!!!!");
    let response_status = res["status"].as_str().unwrap();
    if response_status == "SUCCESS" {
        Some(())
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
}
