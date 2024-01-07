use std::env;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use handlers::volume_handler::{get_current_volume, mute, set_volume, unmute};

use commands::TCPCommand;

use crate::handlers::auxiliary_functions::generate_random_code;

mod commands;
mod handlers;

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