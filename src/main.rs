mod parser;
mod show;

use std::error::Error;

fn open() -> Result<Vec<u8>, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    Ok(std::fs::read(args.get(1).expect("Usage : minipng [file.mp]"))?)
}

fn main() {}
