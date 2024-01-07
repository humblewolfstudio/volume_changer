use rand::distributions::Alphanumeric;
use rand::Rng;
use std::{iter, env};
use std::process::Command;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use commands::TCPCommand;

mod commands;

#[tokio::main]
async fn main() {
    //We collect the args
    let args: Vec<String> = env::args().collect();
    //El primer argumento es simpre algo raro? no se

    let addr = "0.0.0.0:6369";
    let listener = TcpListener::bind(addr).await.unwrap();

    let session_code: String;
    if args.len() > 1 {
        session_code = args[1].clone();
    }else {
        session_code = generate_random_code();
    }

    println!("Listening on {}", addr);
    println!("Session code is: {}", session_code);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let session_code_clone = session_code.clone();
        tokio::spawn(async move {
            process(socket, &session_code_clone).await;
        });
    }
}

async fn process(mut socket: TcpStream, session_id: &str) {
    let mut buf = vec![0; 1024];
    println!("New connection");

    while let Ok(n) = socket.read(&mut buf).await {
        //If the payload is empty, we return the socket
        if n == 0 {
            println!("Payload empty");
            return;
        }

        let socket_message = String::from_utf8(buf[0..n].to_vec()).unwrap();
        println!("Received :{:?}", socket_message);

        let message_array: Vec<&str> = socket_message.split_whitespace().collect();

        if !handle_session_id(message_array[0], session_id) {
            handle_error(&mut socket, "The Session ID is incorrect".to_string()).await;
            return;
        }

        match commands::process_command(message_array[1]) {
            Ok(command) => {
                //data contains the rest of the socket_message
                let data = message_array[2..message_array.len()].to_owned();
                handle_response(&mut socket, command, data).await;
                return;
            }
            Err(e) => {
                handle_error(&mut socket, e).await;
                return;
            }
        }
    }
}

fn handle_session_id(user_sent_id: &str, session_id: &str) -> bool {
    return session_id.eq(user_sent_id);
}

async fn handle_response(socket: &mut TcpStream, command: TCPCommand, data: Vec<&str>) {
    println!("Receivd command: {:?}", command.to_string());
    let response: Vec<u8>;
    match command {
        TCPCommand::GET => {
            response = get_current_volume();
        }
        TCPCommand::SET => {
            response = set_volume(data[0]);
        }
        TCPCommand::MUTE => {
            response = mute();
        }
        TCPCommand::UNMUTE => {
            response = unmute();
        }
    }

    send_response(socket, response).await;
    return;
}

fn get_current_volume() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output()
        .expect("Failed to execute process");
    let response = output.stdout;
    return clear_response(response);
}

fn set_volume(volume: &str) -> Vec<u8> {
    let volume_number: i32 = volume.parse::<i32>().unwrap();

    if volume_number > 100 || volume_number < 0 {
        return "Volume has to be between 0 and 100".as_bytes().to_vec();
    }

    let mut string = "set volume output volume ".to_owned();
    string.push_str(volume);
    let output = Command::new("osascript")
        .arg("-e")
        .arg(string)
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return volume.as_bytes().to_vec();
}

fn mute() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume with output muted")
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return "OK".into();
}

fn unmute() -> Vec<u8> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("set volume without output muted")
        .output()
        .expect("Failed to execute process");
    let _response = output.stdout;
    return "OK".into();
}

async fn handle_error(socket: &mut TcpStream, error: String) {
    //& es para dejar utilizar la variable (borrow)
    println!("Error: {:?}", error.to_string());
    let buf = String::from(error.to_string()).into_bytes();
    socket
        .write_all(&buf[0..buf.len()])
        .await
        .expect("Failed to write error to socket");
    return;
}

async fn send_response(socket: &mut TcpStream, response: Vec<u8>) {
    let debug_message = String::from_utf8(response.to_owned()).unwrap();
    println!("Sending: {:?}", debug_message);
    socket
        .write_all(&response[0..response.len()])
        .await
        .expect("Failed to write error to socket");
    return;
}

fn generate_random_code() -> String {
    let mut rng = rand::thread_rng();
    let code = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(7)
        .collect();
    code
}

fn clear_response(response: Vec<u8>) -> Vec<u8> {
    let string = String::from_utf8(response).unwrap();
    string.replace("\n", "").into_bytes()
}
