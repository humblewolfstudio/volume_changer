use std::process::Command;

use super::auxiliary_functions::clear_response;

pub fn get_current_volume() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output()
        .expect("Failed to execute process");
    let response = output.stdout;
    return clear_response(response);
}

pub fn set_volume(volume: &str) -> Vec<u8> {
    let volume_number: i32 = volume.parse::<i32>().unwrap();

    if volume_number > 100 || volume_number < 0 {
        return "Volume has to be between 0 and 100".as_bytes().to_vec();
    }

    let mut string = "set volume output volume ".to_owned();
    string.push_str(volume);
    let output = Command::new("osascript")
        .arg("-e")
        .arg(string)
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return volume.as_bytes().to_vec();
}

pub fn mute() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume with output muted")
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return "OK".into();
}

pub fn unmute() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume without output muted")
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return "OK".into();
}