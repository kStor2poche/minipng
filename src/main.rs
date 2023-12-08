use std::error::Error;
use std::env;

fn open() -> Result<Vec<u8>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    Ok(std::fs::read(args.get(1).expect("Can't open file"))?)
}

fn main() {
    println!("Hello, world!");
}
