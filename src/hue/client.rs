use std::collections::HashMap;

use log::trace;
use reqwest::{header::HeaderMap, Response};

use super::types::{Event, Group, Light};

pub struct Hue {
    // username: String,
    client_key: String,
    v1_url: String,
    v2_url: String,
    client: reqwest::Client,
}

impl Hue {
    pub fn new(
        hostname: String,
        username: String,
        client_key: String,
    ) -> Result<Hue, reqwest::Error> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let v1_url = format!("http://{}/api/{}", hostname, username);
        let v2_url = format!("https://{}", hostname);
        Ok(Hue {
            // username,
            client_key: username,
            client,
            v1_url,
            v2_url,
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

    async fn get_v2(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(url)
            .header("hue-application-key", &self.client_key)
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
        let url = format!("{}/groups", self.v1_url);
        let response = self.get(&url).await?;
        let json_response: HashMap<String, Group> =
            response.json::<HashMap<String, Group>>().await?;

        Ok(json_response)
    }

    pub async fn lights(&self) -> Result<HashMap<String, Light>, Box<dyn std::error::Error>> {
        let url = format!("{}/lights", self.v1_url);
        let response = self.get(&url).await?;
        let json_response: HashMap<String, Light> =
            response.json::<HashMap<String, Light>>().await?;

        Ok(json_response)
    }

    pub async fn get_event_stream(&self) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let url = format!("{}/eventstream/clip/v2", self.v2_url);
        let response = self.get_v2(&url).await?;

        let json_response = response.json::<Vec<Event>>().await?;

        Ok(json_response)
    }
}
