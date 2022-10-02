use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        process(&mut socket).await;
    }
}

async fn process(socket: &mut TcpStream) {
    let mut buf_reader = BufReader::new(socket);

    let mut data = [0; 1024];
    let req = buf_reader.read_exact(&mut data).await;

    println!("{:?}", data);
}
