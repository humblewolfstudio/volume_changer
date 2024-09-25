use std::process::Command;

use crate::{commands::TCPCommand, handlers::auxiliary_functions::clear_string, handlers::auxiliary_functions::clear_response};
use crate::handlers::auxiliary_functions::string_to_vecu8;

pub fn get_front_most_window2() -> Result<String, String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set mediaApps to {\"QuickTime Player\", \"VLC\", \"Safari\", \"Google Chrome\", \"IINA\"}")
        .arg("-e")
        .arg("tell application \"System Events\"")
        .arg("-e")
        .arg("set runningApps to (name of every process whose background only is false)")
        .arg("-e")
        .arg("end tell")
        .arg("-e")
        .arg("repeat with appName in mediaApps")
        .arg("-e")
        .arg("if runningApps contains (appName as string) then return appName as string")
        .arg("-e")
        .arg("end repeat")
        .arg("-e")
        .arg("return \"\"") // Return empty string if no app is found
        .output();

    match output {
        Ok(out) => return Ok(clear_string(String::from_utf8(out.stderr).unwrap())),
        Err(_e) => return Err("Failed to execute process".to_string()),
    }
}
pub fn get_front_most_window() -> Result<String, String> {
    let media_apps = vec!["QuickTime Player", "VLC",  "IINA"];

    // Run the `ps` command to list running processes
    let output = Command::new("ps")
        .arg("aux") // List all running processes
        .output()
        .expect("Failed to execute ps command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut final_app = "";
    // Iterate over the known media apps and check if they are running
    for app in media_apps {
        if stdout.contains(app) {
            final_app = app;
            println!("App playing a video: {}", app);
            //return Ok(clear_string(app).unwrap());
        }
    }
    return Ok(final_app.to_string());

}
pub fn app_handler(app_name: String, command: TCPCommand) -> Result<Vec<u8>, Vec<u8>> {
    println!("{:?}", app_name);
    match app_name.to_ascii_lowercase().as_str() {
        "vlc" =>  vlc_command(command),
        "iina" =>  iina_command(command),
        "quicktime player" =>  quicktime_command(command),
        "spotify" =>  spotify_command(command),
        _ =>  Err(Vec::from(app_name + &" not supported uwu")),
    }
}

fn vlc_command(command: TCPCommand) -> Result<Vec<u8>, Vec<u8>>{
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"VLC\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("tell application \"VLC\" to previous");
        }
        TCPCommand::PLAY => {
            cmd = String::from("tell application \"VLC\" to play");
        }
        TCPCommand::PLAY_STATUS => {
            cmd = String::from("tell the application \"VLC\" to return its playing");
        }

        _ => return Err(Vec::from("Command not available in VLC".to_string())),
    }
    return base_command_hablder(cmd);
}
fn iina_command(command: TCPCommand) -> Result<Vec<u8>, Vec<u8>> {
    let cmd: String;
    match command {
        TCPCommand::NEXT => {
            cmd = String::from("tell application \"IINA\" to next");
        }
        TCPCommand::PREV => {
            cmd = String::from("");
        }
        _ => return Err(Vec::from("Command not available in IINA".to_string())),
    }
    return base_command_hablder(cmd);
}
fn quicktime_command(command: TCPCommand) ->Result<Vec<u8>, Vec<u8>> {
    let cmd: String;
    match command {
        TCPCommand::PLAY => {
            cmd = String::from("tell application \"QuickTime Player\" to tell document 1 to play");
        },
        TCPCommand::PAUSE => {
            cmd = String::from("tell application \"QuickTime Player\" to tell document 1 to stop");
        },
        _ => return Err(Vec::from("Command not available in QuickTime".to_string())),
    }
    return base_command_hablder(cmd);
}
fn spotify_command(command: TCPCommand) -> Result<Vec<u8>, Vec<u8>>{
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
        _ => return Err(Vec::from("Command not available in Spotiy".to_string())),
    }
    return base_command_hablder(cmd);
}

pub fn base_command_hablder(cmd: String) -> Result<Vec<u8>, Vec<u8>>{
    println!("{}", cmd);
    let output: Result<std::process::Output, std::io::Error> =
        Command::new("osascript").arg("-e").arg(cmd).output();
    //println!("{:?}", output.unwrap().stdout);
    match output {
        Ok(out) => return Ok(clear_response(out.stdout)),
        Err(_e) => return Err(string_to_vecu8("Failed to execute process")),
    }
}
