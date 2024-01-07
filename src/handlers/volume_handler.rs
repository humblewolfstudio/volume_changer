use std::process::Command;

use super::auxiliary_functions::clear_response;

const OS: &str = std::env::consts::OS;

pub fn get_current_volume() -> Vec<u8> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => {
            let output = Command::new("osascript")
                .arg("-e")
                .arg("output volume of (get volume settings)")
                .output()
                .expect("Failed to execute process");
            response = output.stdout;
        }
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return clear_response(response);
}

pub fn set_volume(volume: &str) -> Vec<u8> {
    let volume_number: i32 = volume.parse::<i32>().unwrap();

    if volume_number > 100 || volume_number < 0 {
        return "Volume has to be between 0 and 100".as_bytes().to_vec();
    }

    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => {
            let mut string = "set volume output volume ".to_owned();
            string.push_str(volume);

            let _output = Command::new("osascript")
                .arg("-e")
                .arg(string)
                .output()
                .expect("Failed to execute process");
            response = volume.as_bytes().to_vec();
        }
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return response;
}

pub fn mute() -> Vec<u8> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => {
            let _output = Command::new("osascript")
                .arg("-e")
                .arg("set volume with output muted")
                .output()
                .expect("Failed to execute process");
            response = "OK".into();
        }
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return response;
}

pub fn unmute() -> Vec<u8> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => {
            let _output = Command::new("osascript")
                .arg("-e")
                .arg("set volume without output muted")
                .output()
                .expect("Failed to execute process");
            response = "OK".into();
        }
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return response;
}
