use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn serve(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await?;

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    async_std::task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                log_error(serve(stream).await);
            });
        }
        Ok(())
    })
}

fn log_error(result: std::io::Result<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
