mod parser;
mod show;
mod display;
mod errors;

use std::error::Error;

fn open(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(std::fs::read(path)?)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("Usage : minipng [file.mp]");
    let _data = open(path);
}
