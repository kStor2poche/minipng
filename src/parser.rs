use std::error::Error;

use crate::errors::{*, self};

const MAGIC_BYTES: [u8; 8] = [b'M', b'i', b'n', b'i', b'-', b'P', b'N', b'G']; //Mini-PNG as byte array

// Helper function to read a u32 from an iterator of u8
fn read_u32<T>(data: &mut T) -> u32
where T: Iterator<Item = u8>
{
    data.next().unwrap() as u32 * 2_u32.pow(24)
    + data.next().unwrap() as u32 * 2_u32.pow(16)
    + data.next().unwrap() as u32 * 2_u32.pow(8)
    + data.next().unwrap() as u32
}

pub trait Block {
    fn get_kind(&self) -> char;
    fn get_length(&self) -> u32;
    fn from_raw_data<T>(data: &mut T, block_length: u32) -> Self where Self: Sized, T: Iterator<Item = u8>;
}


pub struct Header {
    width: u32,
    height: u32,
    pixel_type: u8,
}

impl Block for Header {
    fn get_kind(&self) -> char {
        'H'
    }
    fn get_length(&self) -> u32 {
        9
    }
    fn from_raw_data<T>(data: &mut T, _block_length: u32) -> Self where T: Iterator<Item = u8> {
        let width = read_u32(data); 

        let height = read_u32(data); 

        let pixel_type = data.next().unwrap();

        Self { width, height, pixel_type }
    }
}

impl Header {
    pub fn get_content(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.pixel_type)
    }
}

pub struct Comment {
    content: String,
}

impl Block for Comment {
    fn get_kind(&self) -> char {
        'C'
    }
    fn get_length(&self) -> u32 {
        self.content.len() as u32
    }
    fn from_raw_data<T>(data: &mut T, block_length: u32) -> Self where T: Iterator<Item = u8> {
        let content = data.take(block_length as usize).map(|c| c as char).collect();
        Self { content }
    }
}

impl Comment {
    pub fn get_content(&self) -> &String {
        &self.content
    }
}

pub struct DataBlock {
    content: Vec<u8>,
}

impl Block for DataBlock {
    fn get_kind(&self) -> char {
        'D'
    }
    fn get_length(&self) -> u32 {
        self.content.len() as u32
    }
    fn from_raw_data<T>(data: &mut T, block_length: u32) -> Self where Self: Sized, T: Iterator<Item = u8> {
        let content = data.take(block_length as usize).collect();
        Self { content }
    }
}

impl DataBlock {
    pub fn get_content(&self) -> &Vec<u8> {
        &self.content
    }
}


pub fn validate_magic_bytes(input: &Vec<u8>) -> Result<(), MalformedFileError> {
    if input.get(0..8).unwrap() == MAGIC_BYTES {
        Ok(())
    } else {
        Err(MalformedFileError::new("Invalid file format"))
    }
}

pub fn parse_blocks(input: &Vec<u8>) -> Result<(Option<Header>, Vec<Comment>, Vec<DataBlock>), Box<dyn Error>> {
    let mut input_iter = input.iter().map(|e| *e);
    let mut blocks: (Option<Header>, Vec<Comment>, Vec<DataBlock>) = (None, Vec::new(), Vec::new());

    while let Some(b) = input_iter.next() {
        match b {
            b'H' => {
                let block_length = read_u32(&mut input_iter);
                if block_length != 9 {
                    return Err(Box::new(MalformedFileError::new("Wrong header length")))
                }
                if blocks.0.is_some() {
                    return Err(Box::new(MalformedFileError::new("Multiple headers")))
                }
                blocks.0 = Some(Header::from_raw_data(&mut input_iter, block_length));
            },
            b'C' => {
                let block_length = read_u32(&mut input_iter);
                blocks.1.push(Comment::from_raw_data(&mut input_iter, block_length))
            },
            b'D' => {
                let block_length = read_u32(&mut input_iter);
                blocks.2.push(DataBlock::from_raw_data(&mut input_iter, block_length))
            },
            _ => (),
        }
    };
    Ok(blocks)
}

pub fn validate_file_integrity(header: &Option<Header>, data_blocks: &Vec<DataBlock>) -> Result<(), MalformedFileError> {
    if header.is_none() {
        return Err(errors::MalformedFileError::new("Missing header block"));
    }

    let (width, height, pixel_type) = header.as_ref().unwrap().get_content();
    let data_size = data_blocks.iter()
        .map(|block| {
            block.get_length()
        })
        .sum::<u32>();
    let pixel_size = match pixel_type {
        0 => 1,
        _ => return Err(errors::MalformedFileError::new("Invalid pixel type")),
    };

    if data_size < width * height * pixel_size {
        return Err(errors::MalformedFileError::new("Missing data"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{validate_magic_bytes, MAGIC_BYTES};

    use super::parse_blocks;

    #[test]
    fn test_magic_bytes() {
        let mut test_bytes = MAGIC_BYTES.to_vec();
        validate_magic_bytes(&test_bytes).unwrap();

        test_bytes.append(&mut vec![1, 2, 81]);
        validate_magic_bytes(&test_bytes).unwrap();

        test_bytes[0] = 15;
        assert!(validate_magic_bytes(&test_bytes).is_err());

        test_bytes[0] = 0x4d;
        test_bytes[7] = 0;
        assert!(validate_magic_bytes(&test_bytes).is_err());
    }

    #[test]
    fn test_blocks_parsing() {
        let h_block = vec![b'H',0, 0, 0, 9, 0, 0, 0, 42, 0, 0, 0, 42, 0];

        let com = "Ceci est un commentaire";
        let mut c_block = vec![b'C'];
        c_block.extend((com.len() as u32).to_be_bytes());
        c_block.extend(Vec::from(String::from(com).as_bytes()));

        let mut d_block = vec![b'D', 0, 0, 0, 2];
        d_block.extend([0; 42*42]);

        let mut data = h_block;
        data.extend(c_block);
        data.extend(d_block);
        let blocks = parse_blocks(&data).unwrap();
        assert_eq!(blocks.0.unwrap().get_content(), (42, 42, 0));
        assert_eq!((blocks.1)[0].get_content(), &com.to_string());
    }
}
