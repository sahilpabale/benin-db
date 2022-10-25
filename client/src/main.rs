use std::env;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    // creating a tcp stream connected to port `127.0.0.1` on port `6379`
    let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    // initializing the `cmd_from_arg` to a `command` variable 
    let command = cmd_from_arg();
    // writing the command to the tcp stream
    stream.write_all(command.as_bytes()).await.unwrap();
}

// function returning a `String` type
fn cmd_from_arg() -> String {
    // getting the arguments passed in the request in the form of a `Vec<String>` type
    let args: Vec<String> = env::args().collect();

    let args: Vec<String> = args[1..].to_vec();

    // joining the arguments separated by " "
    let command = args.join(" ");
    command
}
