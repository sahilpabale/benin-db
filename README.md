<img width="1920" alt="BeninDB Banner" src="https://user-images.githubusercontent.com/63385587/196757780-87278b93-600e-49fd-8788-7c1a849fd732.png">

# Benin DB
### A Rust implementation of Redis Caching Database from scratch.

### How to use
- #### Server -
  You can the server binary by running
  `cargo run --bin server` in the same directory as `db.json`.

  The server runs on port `6379`
- #### Client -
  After starting the server you can connect to the server from the client binary.
  `cargo run --bin client <cmd>`
  Commands currently supported:

  * `set <key> <value`
  * ~~`get <key>`~~
