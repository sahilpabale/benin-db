#[derive(Debug)]
pub enum Command {
    Get { key: String },
    Set { key: String, val: String },
}
