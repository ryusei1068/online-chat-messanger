use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn serve(mut socket: TcpStream) -> std::io::Result<()> {
    let mut msg_from_client = String::new();
    socket.read_to_string(&mut msg_from_client)?;
    println!("msg from client: {}", msg_from_client);

    socket.write(b"hello")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");
    let listener = TcpListener::bind(address)?;

    let mut new_connections = listener.incoming();

    while let Some(socket_result) = new_connections.next() {
        let socket = socket_result?;
        thread::spawn(|| log_error(serve(socket)));
    }

    Ok(())
}

fn log_error(result: std::io::Result<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
