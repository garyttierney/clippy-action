use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub src_path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
    pub level: String,
    pub spans: Vec<Span>,
    pub rendered: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Span {
    pub line_start: u64,
    pub line_end: u64,
    pub column_start: u64,
    pub column_end: u64,
    pub is_primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompilerMessage {
    pub target: Target,
    pub message: Message,
}
