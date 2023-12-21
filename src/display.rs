use crate::{parser::{Header, Comment, DataBlock}, errors::MalformedFileError};
use std::fmt;

/* 
* Part 2 :
* Implementing fmt::Display for some blocks to print them in a nice way with the `print!` and `println!` macros
* */

// helper function
fn rgb_pixel(r: u8, g: u8, b: u8) -> String{
    format!("\x1b[38;2;{};{};{}m██", r, g, b)
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Comment : {}", self.get_content())
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let content = self.get_content();
                let tmp_err_format;
                let mode = match content.2 {
                    0 => "black and white",
                    1 => "grey level",
    2 => "palette",
                    3 => "24 bit color",
                    n => {
                        tmp_err_format = format!("found invalid mode \"{}\"", n);
                        &tmp_err_format
                    },
                };
    write!(f, "Image info :\n{}x{}, {}.", content.0, content.1, mode)
    }
}


pub fn get_image(header: &Header, data: &Vec<DataBlock>) -> Result<Box<dyn Image>, MalformedFileError> {
    let (width, height, pixel_type) = header.get_content();
    match pixel_type {
        0 => Ok(Box::new(BwImage::from_blocks(data, width, height))),
        1 => Ok(Box::new(GsImage::from_blocks(data, width, height))),
        _ => Err(MalformedFileError::new("Invalid pixel type"))
    }
}


pub struct BwImage {
    data: Vec<bool>,
    width: u32,
    height: u32,
}
pub struct GsImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}
pub struct PalImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}
pub struct RgbImage {
    data: Vec<(u8, u8, u8)>,
    width: u32,
    height: u32,
}

pub trait Image {
    fn from_blocks(blocks: &Vec<DataBlock>, width: u32, height: u32) -> Self where Self: Sized;
    fn display(&self);
}
impl Image for BwImage {
    fn from_blocks(blocks: &Vec<DataBlock>, width: u32, height: u32) -> Self {
        let data = blocks.iter()
                         .map(|data_block| data_block.get_content())
                         .flatten()
                         .map(|c| vec![ c & 0b10000000 == 0b10000000,
                                        c & 0b01000000 == 0b01000000,
                                        c & 0b00100000 == 0b00100000,
                                        c & 0b00010000 == 0b00010000,
                                        c & 0b00001000 == 0b00001000,
                                        c & 0b00000100 == 0b00000100,
                                        c & 0b00000010 == 0b00000010,
                                        c & 0b00000001 == 0b00000001,])
                         .flatten().collect::<Vec<bool>>();
        Self { data, width, height }
    }
    fn display(&self) {
        self.data.chunks_exact(self.width as usize)
                 .take(self.height as usize)
                 .for_each(
                     |r| println!("{}", 
                                  r.iter()
                                   .map(|b| if *b {' '} else {'X'})
                                   .collect::<String>())
                  );
    }
}
impl Image for GsImage {
    fn from_blocks(blocks: &Vec<DataBlock>, width: u32, height: u32) -> Self where Self: Sized {
        let data = blocks.iter()
                         .map(|data_block| data_block.get_content())
                         .flatten()
                         .map(|uchar| *uchar)
                         .collect::<Vec<u8>>();
        Self { data, width, height }
    }
    fn display(&self) {
        self.data.chunks_exact(self.width as usize)
                 .take(self.height as usize)
                 .for_each(
                     |r| println!("{}", 
                                  r.iter()
                                   .map(|gs| rgb_pixel(*gs, *gs, *gs))
                                   .collect::<String>())
                  );
        print!("\x1b[0m")
    }
}
impl Image for PalImage {
    fn from_blocks(blocks: &Vec<DataBlock>, width: u32, height: u32) -> Self where Self: Sized {
        todo!()
    }
    fn display(&self) {
        todo!()
    }
}
impl Image for RgbImage {
    fn from_blocks(blocks: &Vec<DataBlock>, width: u32, height: u32) -> Self where Self: Sized {
        let data = blocks.iter()
                         .map(|data_block| data_block.get_content())
                         .flatten()
                         .map(|uchar| *uchar)
                         .collect::<Vec<u8>>();
        Self { data, width, height }
    }
    fn display(&self) {
        
    }
}
