mod parser;
mod display;
mod errors;

use std::error::Error;

use parser::parse_blocks;

use crate::parser::{validate_magic_bytes, validate_file_integrity};

fn open(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(std::fs::read(path)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get file data
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("Usage : minipng [file.mp]");

    let data = open(path)?;

    // Parse and validate file data
    validate_magic_bytes(&data)?;

    let (header, comments, data_blocks, palette) = parse_blocks(&data)?;


    validate_file_integrity(&header, &data_blocks, &palette)?;

    let img = display::get_image(&header.as_ref().unwrap(), &data_blocks, &palette)?;

    // Display everything
    println!("{}", header.unwrap());
    
    for com in comments {
        println!("{}", com);
    }

    if palette.is_some() {
        println!("{}", palette.as_ref().unwrap())
    }

    img.display();

    Ok(())
}
