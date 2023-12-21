mod parser;
mod display;
mod errors;

use std::error::Error;

use parser::parse_blocks;

use crate::parser::validate_magic_bytes;

fn open(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(std::fs::read(path)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("Usage : minipng [file.mp]");

    let data = open(path)?;

    validate_magic_bytes(&data)?;

    let blocks = parse_blocks(&data)?;

    if blocks.0.is_none() {
        return Err(Box::new(errors::MalformedFileError::new("Missing header block")));
    }

    println!("{}", blocks.0.unwrap());
    
    for com in blocks.1 {
        println!("{}", com);
    }

    Ok(())
}
