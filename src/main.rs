#[macro_use]
extern crate log;
extern crate simplelog;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

mod nanoleaf;

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

    let nanoleaf_client = nanoleaf::client::Nanoleaf::new(
        "http://192.168.1.32:16021",
        "VAE7zu1TMdXcXvbGsV6itpg7vdqYX9TG",
    )
    .unwrap();
    let response = nanoleaf_client.get_panel().await;
    println!("{:?}", response);
    nanoleaf_client.set_brightness(100, 1).await;
}
