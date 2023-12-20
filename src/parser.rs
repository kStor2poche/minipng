use std::error::Error;

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
        let content = data.take(block_length as usize).map(|c| c as char).collect::<String>();
        Self { content }
    }
}

pub struct BwData {
    content: Vec<bool>,
}

impl Block for BwData {
    fn get_kind(&self) -> char {
        'D'
    }
    fn get_length(&self) -> u32 {
        self.content.len() as u32 / 8
    }
    fn from_raw_data<T>(data: &mut T, block_length: u32) -> Self where T: Iterator<Item = u8> {
        let content = data.take(block_length as usize)
                          .map(|c| vec![ c & 0b10000000 == 0b10000000,
                                         c & 0b01000000 == 0b01000000,
                                         c & 0b00100000 == 0b00100000,
                                         c & 0b00010000 == 0b00010000,
                                         c & 0b00001000 == 0b00001000,
                                         c & 0b00000100 == 0b00000100,
                                         c & 0b00000010 == 0b00000010,
                                         c & 0b00000001 == 0b00000001,])
                          .flatten().collect::<Vec<bool>>();
        Self { content }
    }
}

pub fn validate_magic_bytes(input: &Vec<u8>) -> bool {
    input.get(0..8).unwrap() == MAGIC_BYTES
}

pub fn parse_blocks(input: &Vec<u8>) -> Result<Vec<Box<dyn Block>>, Box<dyn Error>> {
    let mut input_iter = input.iter().map(|e| *e);
    let mut res: Vec<Box<dyn Block>> = Vec::new();
    while let Some(b) = input_iter.next() {
        match b {
            b'H' => {
                let block_length = read_u32(&mut input_iter);
                assert!(block_length==9, "Malformed header");
                res.push(Box::new(Header::from_raw_data(&mut input_iter, block_length)));
            },
            b'C' => {
                let block_length = read_u32(&mut input_iter);
                res.push(Box::new(Comment::from_raw_data(&mut input_iter, block_length)))
            },
            b'D' => {},
            _ => (),
        }
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parser::{validate_magic_bytes, MAGIC_BYTES};

    #[test]
    fn test_magic_bytes() {
        let mut test_bytes = MAGIC_BYTES.to_vec();
        assert!(validate_magic_bytes(
            &test_bytes
        ));

        test_bytes.append(&mut vec![1, 2, 81]);
        assert!(validate_magic_bytes(
            &test_bytes
        ));

        test_bytes[0] = 15;
        assert!(!validate_magic_bytes(
            &test_bytes
        ));

        test_bytes[0] = 0x4d;
        test_bytes[7] = 0;
        assert!(!validate_magic_bytes(
            &test_bytes
        ));
    }

    #[test]
    fn test() {}
}
