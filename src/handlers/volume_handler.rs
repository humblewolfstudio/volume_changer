use crate::handlers::auxiliary_functions::{clear_string, string_to_vecu8};
use crate::multimedia_helper::{app_handler, get_front_most_window};

use super::auxiliary_functions::clear_response;
use super::macos_handler::{get_macos_current_volume, mute_macos, set_macos_volume, unmute_macos};

const OS: &str = std::env::consts::OS;

pub fn get_current_volume() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match get_macos_current_volume() {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err),
        },
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(clear_response(response));
}

pub fn set_volume(volume: &str) -> Result<Vec<u8>, Vec<u8>> {
    let volume_number: i32 = volume.parse::<i32>().unwrap();

    if volume_number > 100 || volume_number < 0 {
        return Err("Volume has to be between 0 and 100".as_bytes().to_vec());
    }

    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match set_macos_volume(volume) {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err),
        },
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}

pub fn mute() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match mute_macos() {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err),
        },
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}
pub fn next() -> Vec<u8> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => {
            let app_name = get_front_most_window();
            println!("Front app: {}", app_name);
            let res = app_handler(clear_string(app_name));
            response = string_to_vecu8(&res);
            //exec_a
        }
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return response;
}
pub fn unmute() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match unmute_macos() {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err),
        },
        "windows" => {
            response = "Windows not supported yet".as_bytes().to_vec();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}

pub fn increment() -> Result<Vec<u8>, Vec<u8>> {
    match get_current_volume() {
        Ok(current_volume) => {
            let current_volume_string = String::from_utf8_lossy(&current_volume).to_string();
            let mut volume_int = current_volume_string.parse::<i32>().unwrap_or(0);
            volume_int += 5;
            let volume_string = volume_int.to_string();
            match set_volume(&volume_string) {
                Ok(res) => return Ok(res),
                Err(err) => return Err(err),
            }
        }
        Err(err) => return Err(err),
    }
}

pub fn decrease() -> Result<Vec<u8>, Vec<u8>> {
    match get_current_volume() {
        Ok(current_volume) => {
            let current_volume_string = String::from_utf8_lossy(&current_volume).to_string();
            let mut volume_int = current_volume_string.parse::<i32>().unwrap_or(0);
            volume_int -= 5;
            let volume_string = volume_int.to_string();
            set_volume(&volume_string)
        }
        Err(err) => return Err(err),
    }
}
