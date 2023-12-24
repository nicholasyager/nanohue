use log::trace;
use reqwest::Response;
use serde::{Deserialize, Serialize};

use super::types::{BoolValue, Effect, Panel, TransitionValue};
pub struct Nanoleaf {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct PowerUpdate {
    on: BoolValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct Value<T> {
    value: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct BrightnessUpdate {
    brightness: TransitionValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColorTemperatureUpdate {
    ct: Value<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EffectUpdate {
    write: Effect,
}

#[derive(Debug, Serialize, Deserialize)]
struct EffectSelect {
    select: String,
}

impl Nanoleaf {
    pub fn new(hostname: String, api_token: String) -> Result<Nanoleaf, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let base_url = format!("http://{}:16021/api/v1/{}", hostname, api_token);
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
        trace!(target: "nanoleaf", "Creating PUT payload: {:?}", body);
        let response = self
            .client
            .put(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .map_err(|err| panic!("Failed to get result. {:?}", err))
            .unwrap();

        trace!(
            target: "nanoleaf",
            "Nanoleaf responded with a status code of {:?}",
            response.status().to_string()
        );

        Ok(response)
    }

    #[allow(unused)]
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
            brightness: TransitionValue {
                value: if value <= 100 { value } else { 100 },
                duration,
            },
        };

        trace!(target: "nanoleaf", "Setting the brightness to {} over the next {} seconds.", value, duration);
        let url = format!("{}/state/brightness", self.base_url);

        let _response = self.put(&url, &payload).await?;

        Ok(())
    }

    pub async fn set_color_temperature(
        &self,
        value: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(value >= 1200);
        assert!(value <= 6500);

        let payload = ColorTemperatureUpdate {
            ct: Value::<u32> { value },
        };

        trace!(target: "nanoleaf", "Setting the color temperature to {}.", value);
        let url = format!("{}/state/ct", self.base_url);

        let _response = self.put(&url, &payload).await?;

        Ok(())
    }

    pub async fn write_effect(&self, effect: Effect) -> Result<(), Box<dyn std::error::Error>> {
        // Write an effect to the Nanoleaf, and set it as the active effect.

        trace!(target: "nanoleaf", "Writing effect {:?}.", effect);
        let url = format!("{}/effects", self.base_url);

        let payload = EffectUpdate { write: effect };

        let _response = self.put(&url, &payload).await?;

        Ok(())
    }
}
