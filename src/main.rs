#[macro_use]
extern crate log;
extern crate simplelog;

use std::{collections::HashMap, thread, time};

use log::LevelFilter;
use serde::Deserialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use crate::{
    color::RGBColor,
    hue::types::{Group, Light},
    room::Room,
};

mod color;
mod hue;
mod nanoleaf;
mod room;

#[derive(Debug, Deserialize)]
struct HueConfig {
    host: String,
    group: String,
    username: String,
    client_key: String,
}

#[derive(Debug, Deserialize)]
struct NanoleafConfig {
    host: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct NanoHueConfig {
    hue: HueConfig,
    nanoleaf: NanoleafConfig,
}

fn read_config(path: &str) -> Result<NanoHueConfig, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(path)?;
    let config: NanoHueConfig = serde_yaml::from_reader::<std::fs::File, NanoHueConfig>(f)?;

    Ok(config)
}

fn get_group(groups: HashMap<String, Group>, name: &str) -> Option<Group> {
    for (key, iter_group) in groups {
        if iter_group.name == name {
            println!("Found it! {:?}", iter_group);
            return Some(iter_group);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
    ])
    .unwrap();

    let config = read_config("config.yml").unwrap();

    let hue_client =
        hue::client::Hue::new(config.hue.host, config.hue.username, config.hue.client_key).unwrap();

    let groups = hue_client.groups().await.unwrap();

    let group = match get_group(groups, &config.hue.group) {
        Some(group) => group,
        None => panic!("Unable to locate group `{}` in hue hub.", &config.hue.group),
    };

    let room = Room {
        on: group.action.on,
        brightness: group.action.brightness,
        dynamic: false,
    };

    loop {
        trace!(target: "nanohue", "Looping");

        // Get all lights, and filter them down to just the ones we care about.
        let all_lights = hue_client.lights().await.unwrap();

        let group_lights: Vec<&Light> = group
            .lights
            .iter()
            .map(|light| all_lights.get(light).unwrap())
            .collect();

        println!("{:?}", group_lights);

        for light in group_lights {
            let color_coordinates = light.state.xy.unwrap();
            let color_gamut = light.capabilities.control.color_gamut.unwrap();
            let brightness = light.state.brightness;

            let color = RGBColor::from_coordinate(color_coordinates, color_gamut, brightness);
            println!("{:?}", color);
        }

        // Pause briefly to prevent overloading the nanoleaf.
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
}
