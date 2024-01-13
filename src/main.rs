use std::env;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use handlers::volume_handler::{decrease, get_current_volume, increment, mute, set_volume, unmute, next};

use commands::TCPCommand;

use crate::handlers::auxiliary_functions::{generate_random_code, string_to_vecu8};

mod commands;
mod handlers;
mod multimedia_helper;

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
    } else {
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
        let user_sent_id = message_array[0];
        if user_sent_id != session_id {
            handle_error(&mut socket, string_to_vecu8("The Session ID is incorrect")).await;
            return;
        }

        match commands::process_command(message_array[1]) {
            Ok(command) => {
                //data contains the rest of the socket_message
                let data = message_array[2..message_array.len()].to_owned();
                handle_response(&mut socket, command, data).await;
                return;
            }
            Err(err) => {
                handle_error(&mut socket, string_to_vecu8(&err)).await;
                return;
            }
        }
    }
}

async fn handle_response(socket: &mut TcpStream, command: TCPCommand, data: Vec<&str>) {
    println!("Receivd command: {:?}", command.to_string());
    let response: Vec<u8>;
    let mut error = false;
    match command {
        TCPCommand::GET => match get_current_volume() {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::SET => match set_volume(data[0]) {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::MUTE => match mute() {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::INCREMENT => match increment() {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::DECREASE => match decrease() {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::UNMUTE => match unmute() {
            Ok(res) => response = res,
            Err(err) => {
                error = true;
                response = err;
            }
        },
        TCPCommand::NEXT => {
            response = next();
        }
        TCPCommand::CHILLIN => {
            response = string_to_vecu8("pingiling");
        }
    }

    if error {
        handle_error(socket, response).await
    } else {
        send_response(socket, response).await;
    }

    return;
}

async fn handle_error(socket: &mut TcpStream, error: Vec<u8>) {
    //& es para dejar utilizar la variable (borrow)
    println!(
        "Error: {:?}",
        String::from_utf8(error.clone()).expect("Bytes should be valid utf8")
    );
    socket
        .write_all(&error[0..error.len()])
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
