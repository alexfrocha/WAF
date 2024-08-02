use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

const BASE_URL: &str = "127.0.0.1";

fn handle_request(mut client_stream: TcpStream) {
    let mut buffer = [0; 4096];
    let mut backend_stream: Option<TcpStream> = None;

    match client_stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            // println!("[RECEIVED REQUEST] \n{request:#?}");
            println!("[RECEIVED REQUEST]");
            
            if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
                backend_stream = Some(stream);
            } else {
                eprintln!("[FAILED TO CONNECT BACKEND SERVER]");
                return;
            }

            if let Some(mut backend_stream) = backend_stream {
                if let Err(e) = backend_stream.write_all(&buffer[..size]) {
                    eprintln!("[FAILED TO SEND REQUEST TO BACKEND SERVER] \n{e}");
                    return;
                }
                
                let mut response_buffer = [0; 4096];
                match backend_stream.read(&mut response_buffer) {
                    Ok(response_size) => {
                        let response = String::from_utf8_lossy(&response_buffer[..response_size]);
                        println!("[RECEIVED RESPONSE] \n{response:#?}");
                        if let Err(e) = client_stream.write_all(&response_buffer[..response_size]) {
                            eprintln!("[FAILED TO SEND RESPONSE TO CLIENT] \n{e}");
                        }
                    }
                    Err(e) => eprintln!("[FAILED TO READ RESPONSE FROM BACKED] \n{e}")
                }
            }
        }
        Err(e) => eprintln!("[FAILED TO READ REQUEST FROM CLIENT] \n{e}")
    }

}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;
    println!("[PROXY LISTENING] 127.0.0.1:3030");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_request(stream)
                });
            }
            Err(e) => eprintln!("[FAILED TO ACCEPT CONNECTION] \n{e}")
        }
    }
    Ok(())
}