use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

use serde_json::Value;

pub async fn set(key: String, val: String) {
    let mut dbfile = fs::File::open("./db.json").await.unwrap();
    let mut data = String::new();
    dbfile.read_to_string(&mut data).await.unwrap();
    let mut data: Value = serde_json::from_str(&data).unwrap();
    data[key] = Value::String(val);

    let data = serde_json::to_string(&data).unwrap();

    fs::write("./db.json", data).await.unwrap();
    println!("Done writing!");
}
pub async fn get(key: String) {}
