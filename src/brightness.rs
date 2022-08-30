use lazy_static::lazy_static;
use regex::Regex;
use std::process::Command;

lazy_static! {
    static ref BRIGHTNESS_RE: Regex = Regex::new(r"=(?:\s+)(\d{1,3})")
        .expect("Unable to compile regex for capturing brightness.");
}

pub fn detect_monitors() -> Vec<u8> {
    let capabilities = String::from_utf8(
        Command::new("ddcutil")
            .arg("detect")
            .output()
            .expect("failed to execute 'ddcutil detect' and get its ouput")
            .stdout,
    )
    .expect("Failed to convert output of detect from Vec<u8> to String");
    let mut monitors = Vec::<u8>::new();
    for line in capabilities.lines() {
        if line.contains("Display") {
            monitors.push(
                line.strip_prefix("Display ")
                    .unwrap()
                    .to_owned()
                    .parse()
                    .unwrap(),
            );
        }
    }
    monitors
}

pub fn detect_brightness(monitor: u8) -> (u8, u8) {
    let monitor = format!("{}", monitor);
    let output = Command::new("ddcutil")
        .args(["getvcp", "10", "--display", monitor.as_str()])
        .output()
        .expect(&format!(
            "Unable to get the current brightness for Monitor {monitor}"
        ))
        .stdout;
    let output = String::from_utf8(output)
        .expect("Unable to convert Brightness Output from Vec<u8> to String");
    let mut captures = Vec::<&str>::new();
    for cap in BRIGHTNESS_RE.captures_iter(&output) {
        captures.push(cap.get(1).unwrap().as_str());
    }
    // println!("{captures:#?}");
    let current_value: u8 = captures[0].parse().unwrap();
    let max_value: u8 = captures[1].parse().unwrap();
    (current_value, max_value)
}

pub fn set_brightness(monitor: u8, brightness: u8) {
    Command::new("ddcutil")
        .args([
            "setvcp",
            "10",
            &format!("{brightness}"),
            "--display",
            &format!("{monitor}"),
        ])
        .output()
        .expect(&format!(
            "Unable to set Monitor {} to brightness {}",
            monitor, brightness
        ));
}


#[cfg(test)]
mod tests {
    use super::{detect_brightness, set_brightness, detect_monitors};

    #[test]
    fn test_all(){
        let monitors = detect_monitors();
        assert_eq!(monitors.len(), 2);

        let monitor = monitors[0];
        let brightness = 100u8;
        set_brightness(monitor, brightness);
        let (current_value, _max_value) = detect_brightness(monitor);
        assert_eq!(current_value, brightness);
    }
}