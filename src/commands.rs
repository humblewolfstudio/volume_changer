use core::fmt;

#[derive(Debug)]
pub enum TCPCommand {
    GET,
    SET,
    MUTE,
    UNMUTE,
    CHILLIN,
    INCREMENT,
    DECREASE
}

impl fmt::Display for TCPCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TCPCommand::GET => write!(f, "GET"),
            TCPCommand::SET => write!(f, "SET"),
            TCPCommand::MUTE => write!(f, "MUTE"),
            TCPCommand::UNMUTE => write!(f, "UNMUTE"),
            TCPCommand::CHILLIN => write!(f, "CHILLIN"),
            TCPCommand::INCREMENT => write!(f, "VOL_INCREMENT"),
            TCPCommand::DECREASE => write!(f, "VOL_DECREASE"),
        }
    }
}

pub fn process_command(command: &str) -> Result<TCPCommand, String> {
    match command.to_lowercase().as_ref() {
        "get" => return Ok(TCPCommand::GET),
        "set" => return Ok(TCPCommand::SET),
        "mute" => return Ok(TCPCommand::MUTE),
        "unmute" => return Ok(TCPCommand::UNMUTE),
        "chillin" => return Ok(TCPCommand::CHILLIN),
        "vol_increment" => return Ok(TCPCommand::INCREMENT),
        "vol_decrease" => return Ok(TCPCommand::DECREASE),
        _ => return Err(String::from("Command doesnt exist")),
    }
}
