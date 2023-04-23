use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use redis::{Commands, RedisResult, ConnectionLike, RedisError};
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
fn get_connection() -> redis::RedisResult<redis::Connection> {
    // currently each connection is separate and not pooled.
    // https://github.com/redis-rs/redis-rs/blob/7eab4cf39c5d18c4b7b9ae3f5cffd3e8878cc633/README.md#basic-operation
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let conn = client.get_connection()?;
    Ok(conn)
}

struct Redis_Client;

impl Redis_Client {
    fn add_room<C: ConnectionLike>(conn: &mut C, key: &str) -> redis::RedisResult<bool> {
        // TODO: need some error handling when not connection or failed to store data
        conn.set(key, "")
    }

    fn get_rooms<C: ConnectionLike>(conn: &mut C, key: &str) -> redis::RedisResult<String> {
        // TODO: same as above
        conn.get(key)
    }

    fn is_existing_room<C: ConnectionLike>(conn: &mut C, room_name: &str) -> redis::RedisResult<bool> {
        conn.exists(room_name)
    }
}

struct Handler;

impl Handler {
    fn client_handler(req: Request, peer: SocketAddr) -> Response {
        // TODO: conditional branch by request info(create_room, enter_room, get_room. etc)
        /*
           request kind
           CreateRoom = 1,
           EnterRoom = 2,
           GetRooms = 3,
        */
        let mut conn = get_connection().unwrap();
        match req.kind {
            2 => {
                // TODO:confirm the existing room name
                Response {
                    status: "success".to_string(),
                    detail: format!("created a {:?}", req.room_name),
                }
            }
            3 => {
                Response {
                    status: "success".to_string(),
                    detail: Redis_Client::get_rooms(&mut conn, "rooms").unwrap(),
                }
            },
            _ => Response {
                status: "error".to_string(),
                detail: "invalid request kind".to_string(),
            },
        }
    }
}

async fn serve(mut stream: TcpStream) -> std::io::Result<()> {
    let mut req = String::new();

    stream.read_to_string(&mut req).await?;
    let req: Request = serde_json::from_str(&req).unwrap();
    let peer = stream.peer_addr();

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

#[test]
fn test_invalid_request() {
    let test_request = Request {
        kind: 4,
        room_name: "".to_string(),
        max_capacity: 0,
    };

    let expeced = Response {
        status: "error".to_string(),
        detail: "invalid request kind".to_string(),
    };

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let acutal = Handler::client_handler(test_request, socket);
    assert_eq!(acutal.status, expeced.status);
    assert_eq!(acutal.detail, expeced.detail);
}


use redis_test::{MockCmd, MockRedisConnection};
#[test]
fn test_get_value_by_key() {
    let mut mock_connection = MockRedisConnection::new(vec![
        MockCmd::new(redis::cmd("GET").arg("rooms"), Ok("test01,test02")),
    ]);

    let expected = "test01,test02".to_string();
    let actual = Redis_Client::get_rooms(&mut mock_connection, "rooms").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_is_existing_room() {
    let mut mock_connection = MockRedisConnection::new(vec![
        MockCmd::new(redis::cmd("EXISTS").arg("room01"), Ok("1")),
    ]);

    let actual = Redis_Client::is_existing_room(&mut mock_connection, "room01").unwrap();
    assert_eq!(actual, true);
}

#[test]
fn test_add_room() {
    let mut mock_connection = MockRedisConnection::new(vec![
        MockCmd::new(redis::cmd("SET").arg("room01").arg(""), Ok("1")),
    ]);

    let actual = Redis_Client::add_room(&mut mock_connection, "room01").unwrap();
    assert_eq!(actual, true);
}