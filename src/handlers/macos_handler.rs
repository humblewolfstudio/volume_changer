use std::process::Command;

use super::auxiliary_functions::{clear_response, string_to_vecu8};

pub fn get_macos_current_volume() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output();

    match output {
        Ok(out) => return Ok(clear_response(out.stdout)),
        Err(e) => return Err("Failed to execute process".as_bytes().to_vec()),
    }
}

pub fn set_macos_volume(volume: &str) -> Result<Vec<u8>, Vec<u8>> {
    let mut string = "set volume output volume ".to_owned();
    string.push_str(volume);

    let output = Command::new("osascript").arg("-e").arg(string).output();

    match output {
        Ok(out) => return Ok(clear_response(volume.as_bytes().to_vec())),
        Err(e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}

pub fn mute_macos() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume with output muted")
        .output();

    match output {
        Ok(out) => return Ok(string_to_vecu8("OK")),
        Err(e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}

pub fn unmute_macos() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume without output muted")
        .output();

    match output {
        Ok(out) => return Ok(string_to_vecu8("OK")),
        Err(e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}