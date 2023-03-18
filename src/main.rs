mod cli;
mod monitor;

use crate::{cli::SubCommands, monitor::Monitor};
use clap::Parser;
use cli::Cli;
use ddc_i2c::I2cDeviceEnumerator;
use std::thread;

fn main() {
    let cli = Cli::parse();

    thread::scope(move |s| {
        let monitors: Vec<Monitor> = I2cDeviceEnumerator::new()
            .expect("Failed to iterate over Monitors")
            .map(Monitor::from)
            .collect();
        let cmd = cli.command;
        match cmd {
            SubCommands::Brightness(args) => {
                for mut monitor in monitors {
                    let args = args.clone();
                    s.spawn(move || monitor.handle_brightness(args));
                }
            }
            SubCommands::Contrast(args) => {
                for mut monitor in monitors {
                    let args = args.clone();
                    s.spawn(move || monitor.handle_contrast(args));
                }
            }
            SubCommands::Volume(args) => {
                for mut monitor in monitors {
                    let args = args.clone();
                    s.spawn(move || monitor.handle_volume(args));
                }
            }
            SubCommands::Debug => {
                // println!("cmd={cmd:#?}");
                for mut monitor in monitors {
                    s.spawn(move || monitor.capabilities());
                }
            }
        }
    });
}
