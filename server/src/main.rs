use tokio::{
    io::{AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
};

mod database;
mod types;
use database::*;
use types::Command;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key } => {
                    get(key).await;
                }
                Command::Set { key, val } => {
                    set(key, val).await;
                }
            };
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let newtx = tx.clone();
        tokio::spawn(async move {
            process(&mut socket, newtx).await;
        });
    }
}

async fn process(socket: &mut TcpStream, comms_sender: Sender<Command>) {
    let mut buf_reader = BufReader::new(socket);

    let mut data = vec![];
    let _req = buf_reader.read_buf(&mut data).await;

    let data = String::from_utf8(data).unwrap();

    let mut data = data.split_whitespace();
    let command = data.next().unwrap();
    let args: Vec<&str> = data.collect();

    match command {
        "wait" => {
            use std::time::Duration;
            use tokio::time;

            time::sleep(Duration::from_secs(10)).await;
            println!("wait over");
        }

        "set" => {
            let mut args = args.iter();
            let key = args.next().unwrap().to_string();
            let initial_arg = args.next().unwrap().to_string();

            let val = args.fold(initial_arg, |acc, cur| {
                if acc.chars().nth(0) == Some('"') {
                    if acc.chars().last() == Some('"') {
                        return acc;
                    }
                    let mut acc_ = acc;
                    acc_.push(' ');
                    acc_.push_str(cur);

                    acc_
                } else {
                    cur.to_string()
                }
            });

            comms_sender.send(Command::Set { key, val }).await.unwrap();
            // println!("SET key: {:?} value: {}", key, val);
        }

        _ => {
            println!("Unknown Command");
        }
    }
}
