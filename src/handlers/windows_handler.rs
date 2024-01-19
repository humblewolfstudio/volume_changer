use super::auxiliary_functions::parse_windows_volume;
use std::{env, process::Command};
use winput::{Action, Input, Vk};

pub fn get_windows_current_volume() -> Result<Vec<u8>, Vec<u8>> {
    let exec_path = "SetVol.exe";
    if let Ok(mut cwd) = env::current_dir() {
        cwd.push(&exec_path);

        let output = Command::new(cwd)
            .arg("report")
            .output()
            .expect("Failed to execute process");

        return Ok(
            parse_windows_volume(String::from_utf8_lossy(&output.stdout).to_string()).unwrap(), //TODO FIX UNWRAP
        );
    } else {
        return Err("Cant find current_dir".as_bytes().to_vec());
    }
}

pub fn windows_mute_unmute() -> Vec<u8> {
    let inputs = [
        Input::from_vk(Vk::VolumeMute, Action::Press),
        Input::from_vk(Vk::VolumeMute, Action::Release),
    ];
    winput::send_inputs(inputs);

    return "OK".as_bytes().to_vec();
}

pub fn windows_set_volume(volume: &str) -> Result<Vec<u8>, Vec<u8>> {
    let exec_path = "SetVol.exe";
    if let Ok(mut cwd) = env::current_dir() {
        cwd.push(&exec_path);

        let _output = Command::new(cwd)
            .arg(volume)
            .output()
            .expect("Failed to execute process");
        println!("DONE");
        return Ok(volume.into());
    } else {
        return Err("Cant find current_dir".as_bytes().to_vec());
    }
}

pub fn windows_next() -> Vec<u8> {
    let inputs = [
        Input::from_vk(Vk::NextTrack, Action::Press),
        Input::from_vk(Vk::NextTrack, Action::Release),
    ];
    winput::send_inputs(inputs);

    return "OK".as_bytes().to_vec();
}

pub fn windows_prev() -> Vec<u8> {
    let inputs = [
        Input::from_vk(Vk::PrevTrack, Action::Press),
        Input::from_vk(Vk::PrevTrack, Action::Release),
    ];
    winput::send_inputs(inputs);

    return "OK".as_bytes().to_vec();
}

pub fn windows_stop() -> Vec<u8> {
    let inputs = [
        Input::from_vk(Vk::MediaStop, Action::Press),
        Input::from_vk(Vk::MediaStop, Action::Release),
    ];
    winput::send_inputs(inputs);

    return "OK".as_bytes().to_vec();
}

pub fn windows_play_pause() -> Vec<u8> {
    let inputs = [
        Input::from_vk(Vk::MediaPlayPause, Action::Press),
        Input::from_vk(Vk::MediaPlayPause, Action::Release),
    ];
    winput::send_inputs(inputs);

    return "OK".as_bytes().to_vec();
}
