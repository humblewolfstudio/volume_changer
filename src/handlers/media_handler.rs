use crate::{
    handlers::auxiliary_functions::string_to_vecu8,
    multimedia_helper::{app_handler, get_front_most_window},
};

use super::windows_handler::{windows_next, windows_prev, windows_stop, windows_play_pause};

const OS: &str = std::env::consts::OS;

pub fn next() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match get_front_most_window() {
            Ok(app_name) => {
                println!("Front app: {}", app_name);
                match app_handler(app_name, crate::commands::TCPCommand::NEXT) {
                    Ok(res) => response = string_to_vecu8(&res),
                    Err(err) => return Err(string_to_vecu8(&err)),
                }
            }
            Err(err) => return Err(string_to_vecu8(&err)),
        },
        "windows" => {
            response = windows_next();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}

pub fn prev() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match get_front_most_window() {
            Ok(app_name) => {
                println!("Front app: {}", app_name);
                match app_handler(app_name, crate::commands::TCPCommand::PREV) {
                    Ok(res) => response = string_to_vecu8(&res),
                    Err(err) => return Err(string_to_vecu8(&err)),
                }
            }
            Err(err) => return Err(string_to_vecu8(&err)),
        },
        "windows" => {
            response = windows_prev();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}

pub fn play() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match get_front_most_window() {
            Ok(app_name) => {
                println!("Front app: {}", app_name);
                match app_handler(app_name, crate::commands::TCPCommand::PLAY) {
                    Ok(res) => response = string_to_vecu8(&res),
                    Err(err) => return Err(string_to_vecu8(&err)),
                }
            }
            Err(err) => return Err(string_to_vecu8(&err)),
        },
        "windows" => {
            response = windows_play_pause();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}

pub fn pause() -> Result<Vec<u8>, Vec<u8>> {
    let response: Vec<u8>;
    match OS {
        "linux" => {
            response = "Linux not supported :(".as_bytes().to_vec();
        }
        "macos" => match get_front_most_window() {
            Ok(app_name) => {
                println!("Front app: {}", app_name);
                match app_handler(app_name, crate::commands::TCPCommand::PAUSE) {
                    Ok(res) => response = string_to_vecu8(&res),
                    Err(err) => return Err(string_to_vecu8(&err)),
                }
            }
            Err(err) => return Err(string_to_vecu8(&err)),
        },
        "windows" => {
            response = windows_stop();
        }
        _ => response = "Running on an unknown operating system".as_bytes().to_vec(),
    }

    return Ok(response);
}
