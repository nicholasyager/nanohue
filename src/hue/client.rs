use std::collections::HashMap;

use log::trace;
use reqwest::Response;

use super::types::{Group, Light};

pub struct Hue {
    username: String,
    client_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl Hue {
    pub fn new(
        hostname: String,
        username: String,
        client_key: String,
    ) -> Result<Hue, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let base_url = format!("http://{}/api/{}", hostname, username);
        Ok(Hue {
            username,
            client_key,
            client,
            base_url,
        })
    }

    async fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        trace!(
            target: "hue",
            "Hue responded with a status code of {:?}",
            response.status().to_string()
        );

        Ok(response)
    }

    pub async fn groups(&self) -> Result<HashMap<String, Group>, Box<dyn std::error::Error>> {
        let url = format!("{}/groups", self.base_url);
        let response = self.get(&url).await?;
        let json_response: HashMap<String, Group> =
            response.json::<HashMap<String, Group>>().await?;

        Ok(json_response)
    }

    pub async fn lights(&self) -> Result<HashMap<String, Light>, Box<dyn std::error::Error>> {
        let url = format!("{}/lights", self.base_url);
        let response = self.get(&url).await?;
        let json_response: HashMap<String, Light> =
            response.json::<HashMap<String, Light>>().await?;

        Ok(json_response)
    }
}
