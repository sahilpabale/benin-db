#[derive(Debug)]
pub enum Command {
    // getting values using a `key` as input
    Get { key: String },
    // setting values using a `key` and `val` as input
    Set { key: String, val: String },
}
