use std::process::Command;

use super::auxiliary_functions::{clear_response, string_to_vecu8};

pub fn get_macos_current_volume() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output();

    match output {
        Ok(out) => return Ok(clear_response(out.stdout)),
        Err(_e) => return Err("Failed to execute process".as_bytes().to_vec()),
    }
}

pub fn set_macos_volume(volume: &str) -> Result<Vec<u8>, Vec<u8>> {
    let mut string = "set volume output volume ".to_owned();
    string.push_str(volume);

    let output = Command::new("osascript").arg("-e").arg(string).output();

    match output {
        Ok(_out) => return Ok(clear_response(volume.as_bytes().to_vec())),
        Err(_e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}
pub fn get_mute_status_macos() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")


        .arg("output muted of (get volume settings)")

        .output();

    match output {
        Ok(out) => return Ok(clear_response(out.stdout)),
        Err(_e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}

pub fn mute_macos() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("on getMuteStatus()")
        .arg("-e")
        .arg("output muted of (get volume settings)")
        .arg("-e")
        .arg("end getMuteStatus")
        .arg("-e")
        .arg("on setMute()")
        .arg("-e")
        .arg("set volume with output muted")
        .arg("-e")
        .arg("end setMute")
        .arg("-e")
        .arg("on setUnMute()")
        .arg("-e")
        .arg("set volume without output muted")
        .arg("-e")
        .arg("end setUnMute")
        .arg("-e")
        .arg("if getMuteStatus() then")
        .arg("-e")
        .arg("setUnMute()")
        .arg("-e")
        .arg("else")
        .arg("-e")
        .arg("setMute()")
        .arg("-e")
        .arg("end if")
        .output();

    match output {
        Ok(_out) => return Ok(string_to_vecu8("OK")),
        Err(_e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}

pub fn unmute_macos() -> Result<Vec<u8>, Vec<u8>> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume without output muted")
        .output();

    match output {
        Ok(_out) => return Ok(string_to_vecu8("OK")),
        Err(_e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}