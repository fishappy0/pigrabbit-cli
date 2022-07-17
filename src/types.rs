use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ComplicatedBody<'a> {
    pub(crate) secretapikey: &'a str,
    pub apikey: &'a str,
    pub name: &'a str,
    pub dtype: &'a str,
    pub content: &'a str,
    pub ttl: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleBody<'a> {
    pub secretapikey: &'a str,
    pub apikey: &'a str,
    pub content: &'a str,
    pub ttl: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Keys {
    pub secretapikey: String,
    pub apikey: String,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub dtype: String,
    pub content: String,
    pub ttl: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ProvidedRecord {
    #[serde(flatten)]
    pub record: Record,
    pub id: String,
    pub prio: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct Certificate {
    pub intermediate_certificate: String,
    pub certificate_chain: String,
    pub private_key: String,
    pub public_key: String,
}
