use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;

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
    handle: I2cDeviceDdc
}

impl Monitor {
    pub fn get_brightness(&mut self) -> Option<(u8, u8)> {
        let value = self.handle.get_vcp_feature(Features::Brightness.into());
        match value {
            Ok(v) => Some((v.value() as u8, v.maximum() as u8)),
            Err(_) => None,
        }
    }

    pub fn set_brightness(&mut self, value: Option<u8>) {
        self.handle
            .set_vcp_feature(Features::Brightness.into(), value.unwrap() as u16)
            .expect("Cannot Set Brightness");
    }
}

impl From<I2cDeviceDdc> for Monitor {
    fn from(handle: I2cDeviceDdc) -> Self {
        Self {
            handle
        }
    }
}