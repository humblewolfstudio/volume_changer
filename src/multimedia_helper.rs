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
}

pub fn app_handler(app_name: String, command: TCPCommand) -> Result<String, String> {
    println!("{:?}", app_name);
    match app_name.to_ascii_lowercase().as_str() {
        "vlc" => return vlc_command(command),
        "iina" => return iina_command(command),
        "quicktime player" => return quicktime_command(command),
        "spotify" => return spotify_command(command),
        _ => return Err(app_name + &" not supported uwu"),
    }
}

fn vlc_command(command: TCPCommand) -> Result<String, String> {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"VLC\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return Err("Command not available in VLC".to_string()),
    }
    return base_command_hablder(cmd);
}
fn iina_command(command: TCPCommand) -> Result<String, String> {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"IINA\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return Err("Command not available in IINA".to_string()),
    }
    return base_command_hablder(cmd);
}
fn quicktime_command(command: TCPCommand) -> Result<String, String> {
    let cmd: String;
    match command {
        TCPCommand::PLAY => {
            cmd = String::from("tell application \"QuickTime Player\" to tell document 1 to play");
        },
        TCPCommand::PAUSE => {
            cmd = String::from("tell application \"QuickTime Player\" to tell document 1 to stop");
        },
        _ => return Err("Command not available in QuickTime".to_string()),
    }
    return base_command_hablder(cmd);
}
fn spotify_command(command: TCPCommand) -> Result<String, String> {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"Spotify\" to next track");
        }
        TCPCommand::PREV => {
            cmd = String::from("tell application \"Spotify\" to previous track");
        }
        TCPCommand::PLAY => {
            cmd = String::from("tell application \"Spotify\" to playpause");
        }
        _ => return Err("Command not available in Spotiy".to_string()),
    }
    return base_command_hablder(cmd);
}

pub fn base_command_hablder(cmd: String) -> Result<String, String> {
    let output: Result<std::process::Output, std::io::Error> =
        Command::new("osascript").arg("-e").arg(cmd).output();

    match output {
        Ok(_out) => {
            println!("OK");
            return Ok("OK".to_string());
        }
        Err(e) => {
            println!("Error making command: {:?}", e);
            return Err("Error executing command".to_string());
        }
    }
}
