pub struct Block {
    kind: char,
    length: u32,
    content: Vec<u8>,
}

impl Block {
    pub fn get_kind(&self) -> char {
        self.kind
    }
    pub fn get_content(&self) -> &Vec<u8> {
        &self.content
    }
}

const MAGIC_BYTES: [u8; 8] = [0x4d, 0x69, 0x6e, 0x69, 0x2d, 0x50, 0x4e, 0x47];

pub fn validate_magic_bytes(input: &Vec<u8>) -> bool {
    input.get(0..8).unwrap() == MAGIC_BYTES
}

pub fn parse_blocks(input: &Vec<u8>) -> Vec<Block> {
    todo!()
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
}
