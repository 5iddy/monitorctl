use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;

use crate::cli::SubCmdArgs;

pub enum Features {
    Brightness = 0x10,
    Contrast = 0x12,
    Volume = 0x62,
}

impl From<Features> for u8 {
    fn from(f: Features) -> Self {
        match f {
            Features::Brightness => 0x10u8,
            Features::Contrast => 0x12u8,
            Features::Volume => 0x62u8,
        }
    }
}

pub struct Monitor {
    handle: I2cDeviceDdc,
}

impl Monitor {
    pub fn get_brightness(&mut self) -> Option<(u8, u8)> {
        let value = self.handle.get_vcp_feature(Features::Brightness.into());
        match value {
            Ok(v) => Some((v.value() as u8, v.maximum() as u8)),
            Err(_) => None,
        }
    }

    /// Set Brightness for the monitor
    pub fn set_brightness(&mut self, value: u8) {
        self.handle
            .set_vcp_feature(Features::Brightness.into(), value as u16)
            .expect("Cannot Set Brightness");
    }

    /// Increase Brightness for the monitor
    pub fn increase_brightness(&mut self, value: u8) {
        // get current brightness
        let (current_value, max_value) = self
            .get_brightness()
            .expect("Unable to get brightness from the monitor");

        // add the passed value to the current value
        let target_value = current_value.checked_add(value).map_or(max_value, |value| {
            if value <= max_value {
                value
            } else {
                max_value
            }
        });
        self.set_brightness(target_value);
    }

    /// Decrease Brightness for the monitor
    pub fn decrease_brightness(&mut self, value: u8) {
        let (current_value, _max_value) = self
            .get_brightness()
            .expect("Unable to get brightness from the monitor");

        let target_value = current_value.checked_sub(value).unwrap_or(u8::MIN);

        self.set_brightness(target_value);
    }

    /// Handle brightness commands
    pub fn handle_brightness(&mut self, args: SubCmdArgs) {
        if args.get_current_value {
            let (current_value, _) = self.get_brightness().expect("Unable to get brightness");
            println!("{current_value}");
        } else if args.increase {
            self.increase_brightness(args.value);
        } else if args.decrease {
            self.decrease_brightness(args.value);
        } else {
            self.set_brightness(args.value);
        }
    }

    // Get the capabilities for the 
    pub fn capabilities(&mut self) {
        let cap = self
            .handle
            .capabilities_string()
            .expect("Error occurred while trying to get capabilities");
        let cap =
            String::from_utf8(cap).expect("unable to convert capabilities string to uft-8 string");
        println!("Capabilities:\n{cap}");
        let dio = self.handle.get_vcp_feature(0xC8).unwrap();
        println!("dio: {dio:#?}");
    }

    /// Get Contrast for the monitor
    pub fn get_contrast(&mut self) -> Option<(u8, u8)> {
        let value = self.handle.get_vcp_feature(Features::Contrast.into());
        match value {
            Ok(v) => Some((v.value() as u8, v.maximum() as u8)),
            Err(_) => None,
        }
    }

    /// Set Contrast for the monitor
    pub fn set_contrast(&mut self, value: u8) {
        self.handle
            .set_vcp_feature(Features::Contrast.into(), value as u16)
            .expect("an error occured while trying to set brightness");
    }

    /// Increase Contrast for the monitor
    pub fn increase_contrast(&mut self, value: u8) {
        // get current brightness
        let (current_value, max_value) = self
            .get_contrast()
            .expect("Unable to get Contrast from the monitor");

        // add the passed value to the current value
        let target_value = current_value.checked_add(value).map_or(max_value, |value| {
            if value <= max_value {
                value
            } else {
                max_value
            }
        });

        self.set_contrast(target_value);
    }

    pub fn decrease_contrast(&mut self, value: u8) {
        let (current_value, _max_value) = self
            .get_contrast()
            .expect("Unable to get contrast from the monitor");

        let target_value = current_value.checked_sub(value).unwrap_or(u8::MIN);

        self.set_contrast(target_value);
    }

    pub fn handle_contrast(&mut self, args: SubCmdArgs) {
        if args.get_current_value {
            let (current_value, _) = self.get_contrast().expect("Unable to get brightness");
            println!("{current_value}");
        } else if args.increase {
            self.increase_contrast(args.value);
        } else if args.decrease {
            self.decrease_contrast(args.value);
        } else {
            self.set_contrast(args.value);
        }
    }

    /// Get volume for the monitor
    pub fn get_volume(&mut self) -> Option<(u8, u8)> {
        let value = self.handle.get_vcp_feature(Features::Volume.into());
        match value {
            Ok(v) => Some((v.value() as u8, v.maximum() as u8)),
            Err(_) => None,
        }
    }

    /// Set volume for the monitor
    pub fn set_volume(&mut self, value: u8) {
        self.handle
            .set_vcp_feature(Features::Volume.into(), value as u16)
            .expect("an error occured while trying to set volume");
    }

    /// Increase volume for the monitor
    pub fn increase_volume(&mut self, value: u8) {
        // get current volume
        let (current_value, max_value) = self
            .get_volume()
            .expect("Unable to get volume from the monitor");

        // add the passed value to the current value
        let target_value = current_value.checked_add(value).map_or(max_value, |value| {
            if value <= max_value {
                value
            } else {
                max_value
            }
        });

        self.set_volume(target_value);
    }

    /// Decrease volume by value
    pub fn decrease_volume(&mut self, value: u8) {
        let (current_value, _max_value) = self
            .get_volume()
            .expect("Unable to get volume from the monitor");

        let target_value = current_value.checked_sub(value).unwrap_or(u8::MIN);

        self.set_volume(target_value);
    }

    pub fn handle_volume(&mut self, args: SubCmdArgs) {
        if args.get_current_value {
            let (current_value, _) = self.get_volume().expect("Unable to get volume");
            println!("{current_value}");
        } else if args.increase {
            self.increase_volume(args.value);
        } else if args.decrease {
            self.decrease_volume(args.value);
        } else {
            self.set_volume(args.value);
        }
    }
}

impl From<I2cDeviceDdc> for Monitor {
    fn from(handle: I2cDeviceDdc) -> Self {
        Self { handle }
    }
}
