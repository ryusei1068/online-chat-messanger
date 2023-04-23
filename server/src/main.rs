use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use redis::{Client, Commands, Connection, RedisError, RedisResult};
use serde::ser::Error;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    kind: i32,
    room_name: String,
    max_capacity: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: String,
    detail: String,
}

extern crate redis;
fn get_client() -> redis::RedisResult<redis::Client> {
    // currently each connection is separate and not pooled.
    // https://github.com/redis-rs/redis-rs/blob/7eab4cf39c5d18c4b7b9ae3f5cffd3e8878cc633/README.md#basic-operation
    let client = redis::Client::open("redis://127.0.0.1/")?;
    Ok(client)
}

struct Redis_Client {
    redis_client: redis::Client,
}

impl Redis_Client {
    fn new() -> Redis_Client {
        Redis_Client {
            redis_client: get_client().unwrap(),
        }
    }

    fn add_key_value(&self, key: String, value: String) {
        // TODO: need some error handling when not connection or failed to store data
        let _: RedisResult<()> = self.redis_client.get_connection().unwrap().set(key, value);
    }

    fn get_value_by_key(&self, key: String) -> String {
        // TODO: same as above
        self.redis_client
            .get_connection()
            .unwrap()
            .get(key)
            .unwrap()
    }
}

struct Handler {}

impl Handler {
    fn client_handler(&self, req: Request) {
        // TODO: conditional branch by request info(create_room, enter_room, get_room. etc)
        /*
           request kind
           CreateRoom = 1,
           EnterRoom = 2,
           GetRooms = 3,
        */
        let response = match req.kind {
            1 => print!(""),
            _ => print!(""),
        };

        print!("{:?}", response);
    }
}

async fn serve(mut stream: TcpStream) -> std::io::Result<()> {
    let mut req = String::new();

    stream.read_to_string(&mut req).await?;
    let req: Request = serde_json::from_str(&req).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\nheloo";

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
