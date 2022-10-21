#![allow(unused_imports)]
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};
use serde_json::Value;

// async function to set a `val` to a `key` in a json file
pub async fn set(key: String, val: String) {
    // json file acting as the database
    let mut dbfile = fs::File::open("./db.json").await.unwrap();
    // data we want to write to the dbfile
    let mut data = String::new();

    // reading the `data` from the dbfile
    dbfile.read_to_string(&mut data).await.unwrap();
    // converting the data from `String` to Json
    let mut data: Value = serde_json::from_str(&data).unwrap();
    // setting the `val` to the `key` using json
    data[key] = Value::String(val);
    let data = serde_json::to_string(&data).unwrap();

    // writing the `data` to the dbfile
    fs::write("./db.json", data).await.unwrap();
    // printing success message
    println!("Done writing!");
}

// async function to get a value from the database
// using the `key` as the input
pub async fn get(key: String) {}
