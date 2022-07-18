use serde::Serialize;

use crate::types::*;
const API_URL: &str = "https://porkbun.com/api/json/v3/";
pub struct PRClient {
    pub key: Keys,
    client: reqwest::Client,
}

/// **The domain_name argument for each function is the one that you own on PorkBun.com**
impl PRClient {
    pub fn new(keys: Keys) -> Self {
        let client = reqwest::Client::new();
        Self { key: keys, client }
    }

    /// The `send_request` function that execute the post request with the given body.
    async fn send_request<T: Serialize>(
        client: &mut reqwest::Client,
        url: &str,
        body: T,
    ) -> serde_json::Value {
        let res: serde_json::Value = client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .expect("The request received was not in the JSON format omg!!!! pls contact devs!!!!");
        res.to_owned()
    }

    /// The add_record function adds a DNS Record
    pub async fn add_record(&mut self, domain: &str, record_struct: &Record) -> Option<()> {
        let url = format!("{API_URL}dns/create/{domain}");
        let body = ComplicatedBody {
            secretapikey: &self.key.secretapikey,
            apikey: &self.key.apikey,
            name: &record_struct.name,
            dtype: &record_struct.dtype,
            content: &record_struct.content,
            ttl: record_struct.ttl,
        };
        let res = Self::send_request(&mut self.client, &url, &body).await;
        match res["status"].as_str().unwrap() {
            "SUCCESS" => Some(()),
            _ => None,
        }
    }

    /// This function edits the record based on the domain.
    /// While the id requires you to get the id from the api
    ///
    /// You can use the `retrieve_by_domain_with_id` function with an  empty id
    /// to list all of your records
    pub async fn edit_by_domain_and_id(
        &mut self,
        domain: &str,
        id: &str,
        record_struct: &Record,
    ) -> Option<()> {
        let url = format!("{API_URL}dns/edit/{domain}/{id}");
        let body = ComplicatedBody {
            secretapikey: &self.key.secretapikey,
            apikey: &self.key.apikey,
            name: &record_struct.name,
            dtype: &record_struct.dtype,
            content: &record_struct.content,
            ttl: record_struct.ttl,
        };
        let res = Self::send_request(&mut self.client, &url, &body).await;
        match res["status"].as_str().unwrap() {
            "SUCCESS" => Some(()),
            _ => None,
        }
    }

    /// This function edits the record based on the domain, type and(or) subdomain
    pub async fn edit_by_domain_subdomain_and_type(
        &mut self,
        domain: &str,
        subdomain: &str,
        dtype: &str,
        record_struct: Record,
    ) -> Option<()> {
        let url = format!("{API_URL}dns/editByNameType/{domain}/{dtype}/{subdomain}");
        let body = SimpleBody {
            secretapikey: &self.key.secretapikey,
            apikey: &self.key.apikey,
            content: &record_struct.content,
            ttl: record_struct.ttl,
        };
        let res = Self::send_request(&mut self.client, &url, &body).await;
        match res["status"].as_str().unwrap() {
            "SUCCESS" => Some(()),
            _ => None,
        }
    }

    /// This function deletes the record with the domain name, type and subdomain specified
    pub async fn del_by_type_with_subdomain(
        &mut self,
        dtype: &str,
        domain: &str,
        subdomain: &str,
    ) -> Option<()> {
        let url = format!("{API_URL}dns/deleteByNameType/{domain}/{dtype}/{subdomain}");
        let res = Self::send_request(&mut self.client, &url, &self.key).await;
        match res["status"].as_str().unwrap() {
            "SUCCESS" => Some(()),
            _ => None,
        }
    }

    /// This function deletes the record with the domain name and id specified
    pub async fn del_by_id(&mut self, domain: &str, id: &str) -> Option<()> {
        let url = format!("{API_URL}dns/delete/{domain}/{id}");
        let res = Self::send_request(&mut self.client, &url, &self.key).await;
        match res["status"].as_str().unwrap() {
            "SUCCESS" => Some(()),
            _ => None,
        }
    }

    /// This function retrieve the record information
    /// with the domain name, type and subdomain specified
    pub async fn retreive_by_type_with_subdomain(
        &mut self,
        dtype: &str,
        domain: &str,
        subdomain: &str,
    ) -> Option<Vec<ProvidedRecord>> {
        let url = format!("{API_URL}dns/retrieveByNameType/{domain}/{dtype}/{subdomain}");
        let res = Self::send_request(&mut self.client, &url, &self.key).await;

        match res["status"].as_str().unwrap() {
            "SUCCESS" => res["records"].as_array().map(|c| {
                c.iter()
                    .map(|m| serde_json::from_value(m.to_owned()).unwrap())
                    .collect()
            }),
            _ => None,
        }
    }

    /// This function,
    ///
    /// *with a specified id*: retrieves the information on the specific record that
    /// you wanted to view
    ///
    /// *without a specified id*: lists all the records under that domain name
    pub async fn retreive_by_domain_with_id(
        &mut self,
        domain: &str,
        id: &str,
    ) -> Option<Vec<ProvidedRecord>> {
        let url = format!("{API_URL}dns/retrieve/{domain}/{id}");
        let res = Self::send_request(&mut self.client, &url, &self.key).await;

        match res["status"].as_str().unwrap() {
            "SUCCESS" => res["records"].as_array().map(|c| {
                c.iter()
                    .map(|m| serde_json::from_value(m.to_owned()).unwrap())
                    .collect()
            }),
            _ => None,
        }
    }

    /// This function retrieves all the ssl certificates attatched to the domain specified
    pub async fn retreive_ssl_by_domain(&mut self, domain: &str) -> Option<Certificate> {
        let url = format!("{API_URL}ssl/retrieve/{domain}");
        let body = serde_json::to_string(&self.key).unwrap();
        let res = Self::send_request(&mut self.client, &url, &body).await;

        match res["status"].as_str().unwrap() {
            "SUCCESS" => serde_json::from_value(res).unwrap(),
            _ => None,
        }
    }
}
