use tokio::{
    io::{AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
};

// importing modules
mod database;
mod types;
use database::*;
use types::Command;

#[tokio::main]
async fn main() {
    // creating a tcp listener listening to requests on port `6379`
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // creating a channel sending and recieving data
    let (tx, mut rx) = mpsc::channel(32);

    // spawning a new async task 
    tokio::spawn(async move {
        // looping through the recieved data
        while let Some(cmd) = rx.recv().await {    
            // pattern matching sequence for received data(`cmd` or `args`)
            match cmd {
                // matching the `Get` variant of the `Command` enum with the 
                // `get` function with `key` as the input
                Command::Get { key } => {
                    get(key).await;
                }
                // matching the `Set` variant of the `Command` enum with the
                // `set` function with `key` and `val` as input
                Command::Set { key, val } => {
                    set(key, val).await;
                }
            };
        }
    });

    // looping through creatign a `socket` TcpStream and cloning the `tx` Sender command
    // followed by processing them
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let newtx = tx.clone();
        tokio::spawn(async move {
            process(&mut socket, newtx).await;
        });
    }
}

// handler for the process function used for processing the
// `tx` and the TcpStream `socket`
async fn process(socket: &mut TcpStream, comms_sender: Sender<Command>) {
    // creating a `BufReader` for the `socket` TcpStream
    // that reads the data from the `socket` TcpStream
    let mut buf_reader = BufReader::new(socket);

    // reading a mutable data variable in the buffer
    let mut data = vec![];
    buf_reader
        .read_buf(&mut data)
        .await
        .expect("Couldn't read request.");

    // converting the data from `Vec<u8>` to `String`
    let data = String::from_utf8(data).unwrap();

    // splitting any whitespace
    let mut data = data.split_whitespace();
    // iterates through the data
    let command = data.next().unwrap();
    // collecting the arguments from the data
    let args: Vec<&str> = data.collect();

    // pattern matching sequence for the `command` variable
    match command {
        // matching sequence for the `wait` command
        "wait" => {
            use std::time::Duration;
            use tokio::time;

            // setting the default wait time to 10s
            time::sleep(Duration::from_secs(10)).await;
            println!("wait over");
        }

        // matching sequence for the `set` command
        "set" => {
            let mut args = args.iter();
            // iterating through the `key`
            let key = args.next().expect("No arguments provided").to_string();
            // initial argument in the `key` variable
            let initial_arg = args
                .next()
                .expect("Value not provided to set command")
                .to_string();

            let val = args.fold(initial_arg, |acc, cur| {
                if let Some('"') = acc.chars().nth(0) {
                    if let Some('"') = acc.chars().last() {
                        return acc;
                    }
                    let mut acc = acc;
                    acc.push(' ');
                    acc.push_str(cur);

                    acc
                } else {
                    cur.to_string()
                }
            });

            // sending the `Command` enum variant `Set` with the `key` and `val` as input
            comms_sender
                .send(Command::Set { key, val })
                .await
                .expect("Couldnt send command information");
            // println!("SET key: {:?} value: {}", key, val);
        }

        // matching sequence for an unknown command
        _ => {
            println!("Unknown Command");
        }
    }
}
