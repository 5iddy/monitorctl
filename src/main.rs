#[warn(missing_docs)]
mod cli;
mod brightness;
mod monitor;

use ddc_i2c::I2cDeviceEnumerator;
use monitor::Monitor;
use std::thread;
use cli::Cli;

fn main() {
    Cli::execute();
}  
