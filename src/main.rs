use std::process::Command;
use regex::Regex;
use std::str;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref BRIGHTNESS_RE: Regex = Regex::new(r"=(?:\s+)(\d{1,3})")
        .expect("Unable to compile regex for capturing brightness.");
}


fn main() {
    let args: Vec<String>= env::args().collect();
    let brightness: u8 = args[1].parse().expect("Usage: monitorctl <brightness:u8>");
    // println!("{args:#?}");
    let monitors = detect_capabilities();
    // let brightness = 80;
    println!("Found {} monitors", monitors.len());
    for monitor in monitors {
        println!("Setting brightness to {} for Monitor {}", brightness, monitor);
        set_brightness(monitor, brightness);
        let (current_value, max_value) = detect_brightness(&format!("{}", monitor));
        println!("Current Value: {current_value}, Max Value: {max_value}");
    }
}

fn detect_capabilities() -> Vec<u8> {
    let capabilities = String::from_utf8(Command::new("ddcutil")
        .arg("detect")
        .output()
        .expect("failed to execute 'ddcutil detect' and get its ouput")
        .stdout)
        .expect("Failed to convert output of detect from Vec<u8> to String");
    let mut monitors = Vec::<u8>::new();
    for line in capabilities.lines() {
        if line.contains("Display") {
            monitors.push(line.strip_prefix("Display ").unwrap().to_owned().parse().unwrap());
        }
    }
    monitors
}

fn detect_brightness(monitor: &str) -> (u8,u8) {
    let output = Command::new("ddcutil")
        .args(["getvcp", "10", "--display", monitor])
        .output()
        .expect(&format!("Unable to get the current brightness for Monitor {monitor}"))
        .stdout;
    let output = String::from_utf8(output).expect("Unable to convert Brightness Output from Vec<u8> to String");
    let mut captures = Vec::<&str>::new();
    for cap in BRIGHTNESS_RE.captures_iter(&output){
        captures.push(cap.get(1).unwrap().as_str());
    }
    // println!("{captures:#?}");
    let current_value: u8 = captures[0].parse().unwrap();
    let max_value: u8 = captures[1].parse().unwrap();
    (current_value, max_value)
}

fn set_brightness(monitor: u8, brightness: u8) {
    Command::new("ddcutil")
        .args(["setvcp", "10", &format!("{brightness}"), "--display", &format!("{monitor}")])
        .output()
        .expect(&format!("Unable to set Monitor {} to brightness {}", monitor, brightness));
}