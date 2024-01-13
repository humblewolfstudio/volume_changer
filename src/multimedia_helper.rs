use std::process::Command;

use crate::{commands::TCPCommand, handlers::auxiliary_functions::clear_string};
pub fn get_front_most_window() -> Result<String, String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to set frontApp to first application process whose frontmost is true ")
        .arg("-e")
        .arg("set frontAppName to name of frontApp")
        .arg("-e")
        .arg("log frontAppName")
        .output();

    match output {
        Ok(out) => return Ok(clear_string(String::from_utf8(out.stderr).unwrap())),
        Err(_e) => return Err("Failed to execute process".to_string()),
    }
    /*
    return String::from(
        String::from_utf8(_output.stderr.to_vec()).expect("Our bytes should be valid utf8"),
    );*/
}

pub fn app_handler(app_name: String, command: TCPCommand) -> String {
    println!("{:?}", app_name);
    match app_name.as_str() {
        "VLC" => vlc_command(command),
        "IINA" => iina_command(command),
        "Quicktime Player" => quicktime_command(command),
        "Spotify" => spotify_command(command),
        _ => {}
    }
    return String::from("uwu");
}

fn vlc_command(command: TCPCommand) {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"VLC\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return,
    }
    base_command_hablder(cmd);
}
fn iina_command(command: TCPCommand) {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"IINA\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return,
    }
    base_command_hablder(cmd);
}
fn quicktime_command(command: TCPCommand) {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"QuickTime Player\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return,
    }
    base_command_hablder(cmd);
}
fn spotify_command(command: TCPCommand) {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"Spotify\" to next track");
        }
        TCPCommand::PREV => {
            cmd = String::from("tell application \"Spotify\" to previous track");
        }
        _ => return,
    }
    base_command_hablder(cmd);
}

pub fn base_command_hablder(cmd: String) -> Vec<u8> {
    let response: Vec<u8>;
    let output = Command::new("osascript").arg("-e").arg(cmd).output();

    match output {
        Ok(_out) => {
            println!("OK");
            response = "OK".into()
        }
        Err(e) => {
            println!("Error making command: {:?}", e);
            response = "ERROR".into()
        }
    }
    return response;
}
