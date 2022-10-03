use std::env;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    let command = cmd_from_arg();
    stream.write_all(command.as_bytes()).await.unwrap();
}

fn cmd_from_arg() -> String {
    let args: Vec<String> = env::args().collect();

    let args: Vec<String> = args[1..].to_vec();

    let command = args.join(" ");
    command
}
