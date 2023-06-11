use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS");
    let mut stream = TcpStream::connect(address)?;

    stream.write(b"hi")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    Ok(())
}
