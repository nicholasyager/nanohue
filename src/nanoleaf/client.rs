use log::trace;
use reqwest::Response;
use serde::{Deserialize, Serialize};

use super::types::{BoolValue, Panel, TransitionValue};
pub struct Nanoleaf {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct PowerUpdate {
    on: BoolValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct BrightnessUpdate {
    brightness: TransitionValue,
}

impl Nanoleaf {
    pub fn new(
        hostname: &'static str,
        api_token: &'static str,
    ) -> Result<Nanoleaf, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let base_url = format!("{}/api/v1/{}", hostname, api_token);
        Ok(Nanoleaf {
            client,
            base_url: base_url,
        })
    }

    async fn put<T>(&self, url: &str, payload: &T) -> Result<Response, Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let body = serde_json::to_string(payload)?;
        let response = self
            .client
            .put(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?;
        trace!(
            target: "nanoleaf",
            "Nanoleaf responded with a status code of {:?}",
            response.status().to_string()
        );

        Ok(response)
    }

    pub async fn get_panel(&self) -> Result<Panel, reqwest::Error> {
        let response = self.client.get(&(self.base_url)).send().await?;

        let json_response: Panel = response.json::<Panel>().await?;

        Ok(json_response)
    }

    pub async fn set_power(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let payload = PowerUpdate {
            on: BoolValue { value: value },
        };

        let url = format!("{}/state", self.base_url);

        let _response = self.put(&url, &payload).await?;
        Ok(())
    }

    pub async fn set_brightness(
        &self,
        value: u32,
        duration: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = BrightnessUpdate {
            brightness: TransitionValue { value, duration },
        };

        let url = format!("{}/state/brightness", self.base_url);

        let _response = self.put(&url, &payload).await?;

        Ok(())
    }
}
