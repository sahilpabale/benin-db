use tokio::{
    io::{AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(&mut socket).await;
        });
    }
}

async fn process(socket: &mut TcpStream) {
    let mut buf_reader = BufReader::new(socket);

    let mut data = vec![];
    let _req = buf_reader.read_buf(&mut data).await;

    let data = String::from_utf8(data).unwrap();

    match data.as_str() {
        "wait" => {
            use std::time::Duration;
            use tokio::time;

            time::sleep(Duration::from_secs(10)).await;
            println!("wait over");
        }
        _ => {
            println!("Unknown Command");
        }
    }
}
