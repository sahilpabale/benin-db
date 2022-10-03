use std::env;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();

    stream
        .write_all(&"set name benin-db".as_bytes())
        .await
        .unwrap();
}
