mod cli;
mod monitor;

use crate::{cli::SubCommands, monitor::Monitor};
use clap::Parser;
use cli::Cli;
use ddc_i2c::I2cDeviceEnumerator;
use std::thread;

fn main() {
    let cli = Cli::parse();
    let monitors: Vec<Monitor> = I2cDeviceEnumerator::new()
        .unwrap()
        .map(|monitor| Monitor::from(monitor))
        .collect();

    println!("monitors: {}", monitors.len());

    thread::scope(move |s| {
        let cmd = cli.command;
        for mut monitor in monitors {
            let cmd = cmd.clone();
            s.spawn(move || match cmd {
                SubCommands::Brightness(args) => monitor.handle_brightness(args),
                SubCommands::Contrast(args) => monitor.handle_contrast(args),
                SubCommands::Volume(args) => monitor.handle_volume(args),
                SubCommands::Debug => {
                    println!("args={cmd:#?}");
                    monitor.capabilities()
                }
            });
        }
    });
}
