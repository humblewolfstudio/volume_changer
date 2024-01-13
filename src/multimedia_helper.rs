use std::process::Command;
pub fn get_front_most_window() -> String {
    let _output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to set frontApp to first application process whose frontmost is true ")
        .arg("-e")
        .arg("set frontAppName to name of frontApp")
        .arg("-e")
        .arg("log frontAppName")
        .output()
        .expect("Failed to execute process");
    return String::from(
        String::from_utf8(_output.stderr.to_vec()).expect("Our bytes should be valid utf8"),
    );
}

pub fn app_handler(app_name: String) -> String {
    println!("{:?}", app_name);
    match app_name.as_str() {
        "VLC" => vlc_command(),
        "IINA" => iina_command(),
        "Quicktime Player" => quicktime_command(),
        "Spotify" => spotify_command(),
        _ => {}
    }
    return String::from("uwu");
}

fn vlc_command() {
    let cmd = String::from("tell application \"VLC\" to next");
    base_command_hablder(cmd);
}
fn iina_command() {
    let cmd = String::from("tell application \"IINA\" to next");
    base_command_hablder(cmd);
}
fn quicktime_command() {
    let cmd = String::from("tell application \"QuickTime Player\" to next");
    base_command_hablder(cmd);
}
fn spotify_command() {
    let cmd = String::from("tell application \"Spotify\" to next track");
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
