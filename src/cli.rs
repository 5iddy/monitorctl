// use crate::brightness::{detect_brightness, detect_monitors, set_brightness};
use crate::monitor::Monitor;
use clap::{CommandFactory, ErrorKind, Parser};
use ddc_i2c::I2cDeviceEnumerator;
use std::thread;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(about = "")]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(value_parser, value_name = "Value")]
    pub value: u8,

    /// Adds the value to the current value
    #[clap(short = 'a', long = "add", action)]
    pub add: bool,

    /// Subs the value from the current value
    #[clap(short = 's', long = "sub", action)]
    pub sub: bool,

    /// Subs the value from the current value
    #[clap(short = 'b', long = "brightness", action)]
    pub brightness: bool,

    /// Subs the value from the current value
    #[clap(short = 'c', long = "contrast", action)]
    pub contrast: bool,

    /// Subs the value from the current value
    #[clap(short = 'v', long = "volume", action)]
    pub volume: bool,
}

impl Cli {
    pub fn execute() {
        let args = Cli::parse();
        // let monitors = detect_monitors();
        let monitors: Vec<Monitor> = I2cDeviceEnumerator::new()
            .unwrap()
            .map(|monitor| Monitor::from(monitor))
            .collect();
        thread::scope(|s| {
            for mut monitor in monitors {
                let (current_value, max_value) = monitor
                    .get_brightness()
                    .expect("Unable to get the current brightness values");
                let value = args.value;
                let brightness = match (args.add, args.sub) {
                    (true, false) => {
                        if value > (max_value - current_value) {
                            max_value
                        } else {
                            current_value + value
                        }
                    }
                    (false, true) => {
                        if value > current_value {
                            0u8
                        } else {
                            current_value - value
                        }
                    }
                    (false, false) => {
                        if value >= 100 {
                            100u8
                        } else if value == 0 {
                            0u8
                        } else {
                            value
                        }
                    }
                    (true, true) => {
                        let mut cmd = Cli::command();
                        cmd.error(
                            ErrorKind::ArgumentConflict,
                            "cannot use both --add and --sub together",
                        )
                        .exit();
                    }
                };
                s.spawn(move || monitor.set_brightness(Some(brightness)));
            }
        });
    }
}
