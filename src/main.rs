#[macro_use]
extern crate log;
extern crate simplelog;

use std::{collections::HashSet, str::FromStr, thread, time};

use color::{HSVColor, Palette};
use hue::client::Hue;
use log::LevelFilter;
use nanoleaf::{
    client::Nanoleaf,
    types::{Effect, Range},
};
use serde::Deserialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use crate::{
    color::RGBColor,
    hue::types::{EventMessage, Light},
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

async fn get_palette(lights: &Vec<Light>) -> Palette {
    println!("{:?}", lights);

    let mut palette = Palette::new();

    for light in lights {
        let color = light.color.unwrap();
        let color_coordinates = color.xy;
        let color_gamut = color.gamut.unwrap();
        let brightness = light.dimming.brightness;

        let color = RGBColor::from_coordinate(color_coordinates, color_gamut, brightness);
        palette.insert(color.to_hsv());
    }

    palette
}

async fn write_room_to_nanoleaf(nanoleaf_client: &Nanoleaf, room: &mut Room) {
    // Write the room state to the nanoleaf

    // On State
    let _ = nanoleaf_client.set_power(room.on).await;

    // Brightness
    let _ = nanoleaf_client
        .set_brightness(room.brightness.clamp(0.0, 100.0) as u32, 1)
        .await;

    let palette = room.palette.clone().unwrap();
    room.has_updated = false;
    // Effect

    if room.scene_has_updated {
        let animation_type = if room.dynamic {
            String::from_str("random").unwrap()
        } else {
            String::from_str("flow").unwrap()
        };

        let effect = Effect {
            command: String::from_str("display").unwrap(),
            animation_name: String::from_str("hue").unwrap(),
            color_type: String::from_str("HSB").unwrap(),
            animation_data: None,
            brightness_range: Range {
                min: 25,
                max: room.brightness.clamp(0.0, 100.0) as u32,
            },
            loop_animation: true,
            animation_type: animation_type,
            transition_time: if room.dynamic {
                Range { min: 15, max: 30 }
            } else {
                Range { min: 30, max: 60 }
            },
            delay_time: if room.dynamic {
                Range { min: 30, max: 60 }
            } else {
                Range { min: 60, max: 90 }
            },
            palette: palette.iter().cloned().collect::<Vec<HSVColor>>(),
        };

        let _ = nanoleaf_client.write_effect(effect.clone()).await;
        // let _ = nanoleaf_client.set_effect(effect.animation_name).await;
        room.scene_has_updated = false;
    };
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

    let hue_client = Hue::new(config.hue.host, config.hue.username).unwrap();

    let nanoleaf = Nanoleaf::new(config.nanoleaf.host, config.nanoleaf.token).unwrap();

    let rooms = hue_client.rooms().await.unwrap();

    let hue_room: &crate::hue::types::Room = rooms
        .iter()
        .filter(|&r| r.metadata.name == config.hue.group)
        .nth(0)
        .unwrap();
    trace!(
        target: "nanohue",
        "Found the room defined in the configuration. {:?}",
        hue_room
    );

    let room_devices: HashSet<&str> = hue_room
        .children
        .iter()
        .filter(|&child| child.resource_type == "device")
        .map(|device| device.id.as_str())
        .collect();

    let group_resource = hue_room
        .services
        .iter()
        .filter(|&r| r.resource_type == "grouped_light")
        .nth(0)
        .unwrap();

    trace!(
            target: "nanohue",
            "Found the room's group resource. {:?}",
            group_resource
    );

    let group = hue_client.group(&group_resource.id).await.unwrap();
    trace!(
            target: "nanohue",
            "Found the room's grouped light. {:?}",
            group
    );

    let all_lights = hue_client.lights().await.unwrap();
    let lights = all_lights
        .into_iter()
        .filter(|light| room_devices.contains(light.owner.id.as_str()))
        .collect();

    let mut room = Room {
        on: group.on.on,
        brightness: group.dimming.brightness,
        dynamic: false,
        palette: Some(get_palette(&lights).await),
        has_updated: true,
        scene_has_updated: true,
    };

    trace!(target: "nanohue", "Generated a baseline room. {:?}", room);
    write_room_to_nanoleaf(&nanoleaf, &mut room).await;

    loop {
        trace!(target: "nanohue", "Looping");

        let event = hue_client.get_event_stream().await.unwrap();

        let event_data: Vec<EventMessage> =
            event.into_iter().map(|item| item.data).flatten().collect();

        let allowed_types = vec!["grouped_light", "light", "scene"];

        for item in event_data {
            let message_type = item.message_type.as_str();

            if !allowed_types.contains(&message_type) {
                continue;
            }

            if message_type == "grouped_light" {
                // Check if the on status has changed, and if so, write it to the room.
                match &item.on {
                    Some(on) => {
                        room.on = on.on;
                        room.has_updated = true
                    }
                    None => {}
                }

                // Check if the brightness has changed, and if so, write it to the room.
                match &item.dimming {
                    Some(dimming) => {
                        room.brightness = dimming.brightness;
                        room.has_updated = true
                    }
                    None => {}
                }
            } else if message_type == "scene" {
                // Check the scene change! If it is part of our room, grab the new palette

                let scene = hue_client.scene(&item.id).await.unwrap();

                if scene.group.id != hue_room.id || scene.status.active == "inactive" {
                    continue;
                }

                // Construct a color palette from the scene. For this, we have to iterate through
                // the actions to get the light's color gamut, the color, and the brightness. Then,
                // we can compute an HSVColor and store that in the room's palette.

                let mut palette = Palette::new();

                for action in scene.actions {
                    let light = hue_client.light(&action.target.id).await.unwrap();

                    let color = RGBColor::from_coordinate(
                        action.action.color.xy,
                        light.color.unwrap().gamut.unwrap(),
                        action.action.dimming.brightness,
                    );
                    palette.insert(color.to_hsv());
                }

                room.dynamic = scene.status.active == "dynamic_palette";
                room.palette = Some(palette);
                room.scene_has_updated = true;
                room.has_updated = true;
            }

            println!("{:?}", item);
        }

        if room.has_updated {
            write_room_to_nanoleaf(&nanoleaf, &mut room).await;
        }

        // Pause briefly to prevent overloading the nanoleaf.
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}
