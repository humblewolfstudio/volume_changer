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

        match parse_windows_volume(String::from_utf8_lossy(&output.stdout).to_string()) {
            Ok(res) => return Ok(res),
            Err(err) => return Err(err),
        }
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
    let current_volume;

    match get_windows_current_volume() {
        Ok(res) => {
            let vol = res;
            let vol_str = String::from_utf8(vol).unwrap();
            current_volume = vol_str;
        }
        Err(err) => return Err(err),
    }

    match windows_change_volume(
        current_volume.parse::<i32>().unwrap(),
        volume.parse::<i32>().unwrap(),
    ) {
        Ok(_res) => return Ok(volume.into()),
        Err(err) => return Err(err),
    }
}

pub fn windows_change_volume(
    current_volume: i32,
    objective_volume: i32,
) -> Result<Vec<u8>, Vec<u8>> {
    let mut difference = objective_volume - current_volume;
    /* DEBUG COMMENTS
        println!("Current of volume: {:?}", current_volume);
        println!("Objective of volume: {:?}", objective_volume);
        println!("Difference of volume: {:?}", difference);
    */
    let mut inputs = Vec::new();

    if difference % 2 != 0 {
        if difference.is_positive() {
            difference += 1
        } else {
            difference -= 1;
        }
    }

    if current_volume + difference > objective_volume {
        difference -= 2; //Amb aixo ens encarreguem de si la liat i anava a ser superior a 100
    }

    //We divide by 2 because somehow it doubles it when changing volumes
    if difference > 0 {
        let value = difference / 2;
        for _ in 0..value {
            inputs.push(Input::from_vk(Vk::VolumeUp, Action::Press));
            inputs.push(Input::from_vk(Vk::VolumeUp, Action::Release));
        }
    } else if difference < 0 {
        let value = difference.abs() / 2;
        for _ in 0..value {
            inputs.push(Input::from_vk(Vk::VolumeDown, Action::Press));
            inputs.push(Input::from_vk(Vk::VolumeDown, Action::Release));
        }
    }

    winput::send_inputs(inputs);

    return Ok("OK".into());
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
