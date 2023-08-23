pub mod parser;
pub mod token;
pub mod cursor;

use std::{env::args, path::PathBuf, fs};

use parser::Parser;
use tokio::main;
use serde_json::to_string_pretty;

#[main]
pub async fn main() {
    let path = args().nth(1).unwrap();
    let path = PathBuf::from(path);
    let content = fs::read_to_string(path).unwrap();
    let tokens = Parser::new(content).parse();

    println!("Content:\n{}", to_string_pretty(&tokens).unwrap());
}
