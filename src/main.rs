use std::{
    collections::HashMap, 
    net::SocketAddr, 
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener, 
    sync::broadcast
};

mod user;
use user::User;

mod process;
use process::process_result;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap_or_else(|_|panic!("Unable to bind tcplistener"));
    let (tx, _rx) = broadcast::channel(10);
    let users_arc: Arc<Mutex<HashMap<SocketAddr, User>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let users = users_arc.clone();
        tokio::spawn( async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        let to_print = process_result(msg, other_addr, &users);

                        if addr != other_addr {
                            writer.write_all(&to_print.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
