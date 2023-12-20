use std::error::Error;

// Helper function to read a u32 from an iterator of u8
fn read_u32<T>(data: &T) -> u32
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
}

impl Header {
    fn from_raw_data<T>(data: &T) -> Self where T: Iterator<Item = u8> {
        let width = read_u32(data); 

        let height = read_u32(data); 

        let pixel_type = data.next().unwrap();

        Self { width, height, pixel_type }
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
}

impl Comment {
    fn from_raw_data<T>(data: &T, block_length: u32) -> Self where T: Iterator<Item = u8> {
        let content = data.take(block_length as usize).map(|c| c.into(char)).collect::<String>();
        Self { content }
    }
}

const MAGIC_BYTES: [u8; 8] = [0x4d, 0x69, 0x6e, 0x69, 0x2d, 0x50, 0x4e, 0x47]; //Mini-PNG as byte array

pub fn validate_magic_bytes(input: &Vec<u8>) -> bool {
    input.get(0..8).unwrap() == MAGIC_BYTES
}

pub fn parse_blocks(input: &Vec<u8>) -> Result<Vec<Box<dyn Block>>, Box<dyn Error>> {
    let mut input_iter = input.iter().map(|e| *e);
    let mut res: Vec<Box<dyn Block>> = Vec::new();
    while let Some(b) = input_iter.next() {
        match b {
            b'H' => {
                let block_length = read_u32(&input_iter);
                assert!(block_length==9, "Malformed header");
                res.push(Box::new(Header::from_raw_data(&input_iter)))
            },
            b'C' => {},
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
