use log::trace;
use reqwest::Response;

use super::types::{Event, GroupedLight, HueResponse, Light, Room, Scene};

pub struct Hue {
    // username: String,
    client_key: String,
    v2_url: String,
    client: reqwest::Client,
}

impl Hue {
    pub fn new(hostname: String, username: String) -> Result<Hue, reqwest::Error> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let v2_url = format!("https://{}", hostname);
        Ok(Hue {
            // username,
            client_key: username,
            client,
            v2_url,
        })
    }

    async fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        trace!(
            target: "hue",
            "GET {:?}",
            url
        );

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

    pub async fn rooms(&self) -> Result<Vec<Room>, Box<dyn std::error::Error>> {
        let url = format!("{}/clip/v2/resource/room", self.v2_url);
        let response = self.get(&url).await?;
        let json_response: HueResponse<Room> = response.json::<HueResponse<Room>>().await?;

        Ok(json_response.data)
    }

    pub async fn group(&self, id: &str) -> Result<GroupedLight, Box<dyn std::error::Error>> {
        let url: String = format!("{}/clip/v2/resource/grouped_light/{}", self.v2_url, id);
        let response = self.get(&url).await?;
        let json_response: HueResponse<GroupedLight> =
            response.json::<HueResponse<GroupedLight>>().await?;

        let item = json_response.data.iter().nth(0).unwrap();

        Ok(item.clone())
    }

    pub async fn lights(&self) -> Result<Vec<Light>, Box<dyn std::error::Error>> {
        let url = format!("{}/clip/v2/resource/light", self.v2_url);
        let response = self.get(&url).await?;
        let json_response: HueResponse<Light> = response.json::<HueResponse<Light>>().await?;

        Ok(json_response.data.clone())
    }

    pub async fn light(&self, id: &str) -> Result<Light, Box<dyn std::error::Error>> {
        let url: String = format!("{}/clip/v2/resource/light/{}", self.v2_url, id);
        let response = self.get(&url).await?;
        let json_response: HueResponse<Light> = response.json::<HueResponse<Light>>().await?;

        let item = json_response.data.iter().nth(0).unwrap();

        Ok(item.clone())
    }

    pub async fn scene(&self, id: &str) -> Result<Scene, Box<dyn std::error::Error>> {
        let url: String = format!("{}/clip/v2/resource/scene/{}", self.v2_url, id);
        let response = self.get(&url).await?;
        let json_response: HueResponse<Scene> = response.json::<HueResponse<Scene>>().await?;

        let item = json_response.data.iter().nth(0).unwrap();

        Ok(item.clone())
    }

    pub async fn get_event_stream(&self) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let url = format!("{}/eventstream/clip/v2", self.v2_url);
        let response = self.get(&url).await?;

        let json_response = response.json::<Vec<Event>>().await?;

        Ok(json_response)
    }
}
